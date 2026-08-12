// Render a captured terminal frame (tmux capture-pane -p -e output) to PNG.
// Pure-std Go: only golang.org/x/image is required for TTF glyph rasterizing.
// The ANSI parser replays SGR + erase-to-EOL + \r overwrites linearly, the
// same model a real terminal uses, so the PNG matches the live screen.
//
// Usage:
//
//	go run . <frame.raw> <font-path.ttf> <out.png>
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"io"
	"os"
	"strconv"
	"strings"

	"golang.org/x/image/font"
	"golang.org/x/image/font/opentype"
	"golang.org/x/image/math/fixed"
)

const (
	fontSizePx = 21
	lineScale  = 1.16
	padLeftPx  = 24
	padTopPx   = 18
	charPadPx  = 2
	dimScalar  = 0.55
)

// campbell is the stock Windows Terminal scheme: the default target, since
// the demo is captured against a stock WSL profile. Use --scheme to match
// any other terminal.
var campbell = theme{
	bg: hex("#0C0C0C"), fg: hex("#CCCCCC"),
	ansi: [16]color.RGBA{
		hex("#0C0C0C"), hex("#C50F1F"), hex("#13A10E"), hex("#C19C00"),
		hex("#0037DA"), hex("#881798"), hex("#3A96DD"), hex("#CCCCCC"),
		hex("#767676"), hex("#E74856"), hex("#16C60C"), hex("#F9F1A5"),
		hex("#3B78FF"), hex("#B4009E"), hex("#61D6D6"), hex("#F2F2F2"),
	},
}

type theme struct {
	bg   color.RGBA
	fg   color.RGBA
	ansi [16]color.RGBA
}

var tm = campbell

type cell struct {
	ch     rune
	fg     color.RGBA
	bg     color.RGBA
	hasBg  bool
	noText bool
}

func main() {
	args := os.Args[1:]
	if len(args) != 3 && len(args) != 4 {
		fmt.Fprintln(os.Stderr, "usage: render_frame.go <frame.raw> <font.ttf> <out.png> [scheme.json]")
		os.Exit(2)
	}
	if len(args) == 4 {
		tm = loadScheme(args[3])
	}

	raw, err := os.ReadFile(args[0])
	die(err)
	fontBytes, err := os.ReadFile(args[1])
	die(err)
	out, err := os.Create(args[2])
	die(err)
	defer out.Close()

	face, err := newFace(fontBytes, fontSizePx)
	die(err)
	defer face.Close()

	grid := parse(raw)
	img, err := paint(grid, face)
	die(err)
	die(png.Encode(out, img))
	fmt.Println("wrote", args[2])
}

func loadScheme(path string) theme {
	raw, err := os.ReadFile(path)
	die(err)
	var fj struct {
		Foreground string   `json:"foreground"`
		Background string   `json:"background"`
		AnsiColor  []string `json:"ansiColor"`
	}
	die(json.Unmarshal(raw, &fj))
	th := campbell
	if fj.Foreground != "" {
		th.fg = hex(fj.Foreground)
	}
	if fj.Background != "" {
		th.bg = hex(fj.Background)
	}
	for i, c := range fj.AnsiColor {
		if i < 16 && strings.TrimSpace(c) != "" {
			th.ansi[i] = hex(strings.TrimSpace(c))
		}
	}
	return th
}

type faceT struct {
	font.Face
	advance int
	lineH   int
}

func newFace(ttf []byte, px float64) (*faceT, error) {
	f, err := opentype.Parse(ttf)
	if err != nil {
		return nil, err
	}
	fc, err := opentype.NewFace(f, &opentype.FaceOptions{Size: px, DPI: 72, Hinting: font.HintingFull})
	if err != nil {
		return nil, err
	}
	a, _ := fc.GlyphAdvance('M')
	return &faceT{Face: fc, advance: a.Round() + charPadPx, lineH: int(px * lineScale)}, nil
}

func paint(grid [][]cell, f *faceT) (image.Image, error) {
	grid = trim(grid)
	if len(grid) == 0 {
		return nil, fmt.Errorf("empty frame")
	}
	cols := 0
	for _, row := range grid {
		if n := cellWidth(row); n > cols {
			cols = n
		}
	}
	rows := len(grid)
	w, h := padLeftPx+cols*f.advance+padLeftPx, padTopPx+rows*f.lineH+padTopPx
	img := image.NewRGBA(image.Rect(0, 0, w, h))
	clear(img, tm.bg)

	d := &font.Drawer{}
	for y, row := range grid {
		x := 0
		for _, c := range row {
			if c.hasBg {
				fillRect(img, padLeftPx+x*f.advance, padTopPx+y*f.lineH, f.advance, f.lineH, c.bg)
			}
			if !c.noText && c.ch != 0 && c.ch != ' ' {
				d.Src = image.NewUniform(c.fg)
				d.Face = f
				d.Dst = img
				d.Dot = fixed.P(padLeftPx+x*f.advance, padTopPx+y*f.lineH+int(float64(f.lineH)*0.80))
				d.DrawString(string(c.ch))
			}
			x += charCells(c.ch)
		}
	}
	return img, nil
}

func cellWidth(row []cell) int {
	n := 0
	for _, c := range row {
		n += charCells(c.ch)
	}
	return n
}

func trim(grid [][]cell) [][]cell {
	lastRow := -1
	for y, row := range grid {
		filled := false
		for _, c := range row {
			if c.ch != 0 && c.ch != ' ' || c.hasBg {
				filled = true
				break
			}
		}
		if filled {
			lastRow = y
		}
	}
	if lastRow < 0 {
		return nil
	}
	grid = grid[:lastRow+1]
	cols := 0
	for _, row := range grid {
		w := 0
		for i := len(row) - 1; i >= 0; i-- {
			if row[i].ch != 0 && row[i].ch != ' ' {
				w = i + 1
				break
			}
		}
		if w > cols {
			cols = w
		}
	}
	for y := range grid {
		if len(grid[y]) > cols {
			grid[y] = grid[y][:cols]
		}
	}
	return grid
}

func charCells(r rune) int {
	if r >= 0x2500 && r <= 0x257F {
		return 2
	}
	return 1
}

func clear(img *image.RGBA, c color.RGBA) {
	for i := 0; i < len(img.Pix); i += 4 {
		img.Pix[i], img.Pix[i+1], img.Pix[i+2], img.Pix[i+3] = c.R, c.G, c.B, 0xff
	}
}

func fillRect(img *image.RGBA, x, y, w, h int, c color.RGBA) {
	for j := y; j < y+h; j++ {
		for i := x; i < x+w; i++ {
			img.SetRGBA(i, j, c)
		}
	}
}

// --- ANSI replay ---

type style struct {
	attrs string // "", "1", "2"
	fg    color.RGBA
	base  int // 0..7 when fg is an ansi base color, else -1 (primes the bold->bright swap)
}

func (s style) resolve() color.RGBA {
	fg := s.fg
	if s.attrs == "1" && s.base >= 0 && s.base < 8 {
		fg = tm.ansi[s.base+8]
	}
	if s.attrs == "2" {
		fg = blend(fg, tm.bg, dimScalar)
	}
	return fg
}

func blend(fg, bg color.RGBA, k float64) color.RGBA {
	mix := func(a, b uint8) uint8 { return uint8(float64(a)*(1-k) + float64(b)*k) }
	return color.RGBA{mix(fg.R, bg.R), mix(fg.G, bg.G), mix(fg.B, bg.B), 0xff}
}

func parse(raw []byte) [][]cell {
	var grid [][]cell
	var row []cell
	rewind := -1 // column marker for overwrite-ahead (\r)
	col := 0
	st := style{fg: tm.fg, base: -1}

	r := bytes.NewReader(raw)
	for {
		b, err := r.ReadByte()
		if err == io.EOF {
			break
		}
		die(err)
		switch {
		case b == '\n':
			grid = append(grid, row)
			row = nil
			rewind, col = -1, 0
		case b == '\r':
			rewind, col = 0, 0
		case b == 0x1b:
			csi, final, err := readCSI(r, b)
			die(err)
			switch final {
			case 'm':
				st = applySGR(st, csi)
			case 'K':
				if rewind == -1 {
					if len(row) > col {
						row = row[:col]
					}
				}
			case 'A', 'B', 'C', 'D', 'H', 'J', 'h', 'l', 's', 'u':
				// cursor moves / modes already folded into capture
			}
		default:
			ch := rune(b)
			if b >= 0x80 {
				r.UnreadByte()
				ch, _, err = r.ReadRune()
				if err != nil {
					ch = rune(b)
				}
			}
			put := cell{ch: ch, fg: st.resolve(), hasBg: false}
			if rewind >= 0 {
				if len(row) <= col {
					row = append(row, cell{})
				}
				row[col] = put
				col++
			} else {
				row = append(row, put)
				col++
			}
		}
	}
	grid = append(grid, row)
	if len(grid) > 0 && len(grid[len(grid)-1]) == 0 {
		grid = grid[:len(grid)-1]
	}
	return grid
}

func readCSI(r *bytes.Reader, _ byte) (string, byte, error) {
	var params []byte
	for {
		b, err := r.ReadByte()
		if err != nil {
			return string(params), 0, err
		}
		if b >= 0x40 && b <= 0x7e && b != '[' && b != ']' {
			return string(params), b, nil
		}
		if b == '[' || b == ']' {
			continue
		}
		params = append(params, b)
	}
}

func applySGR(st style, params string) style {
	if params == "" {
		params = "0"
	}
	args := strings.Split(params, ";")
	for i := 0; i < len(args); i++ {
		switch args[i] {
		case "0":
			st = style{fg: tm.fg, base: -1}
		case "1":
			st.attrs = "1"
		case "2":
			st.attrs = "2"
		case "22":
			st.attrs = ""
		case "30", "31", "32", "33", "34", "35", "36", "37":
			n, _ := strconv.Atoi(args[i])
			st.fg, st.base = tm.ansi[n-30], n-30
		case "90", "91", "92", "93", "94", "95", "96", "97":
			n, _ := strconv.Atoi(args[i])
			st.fg, st.base = tm.ansi[8+n-90], -1
		case "38":
			if i+2 < len(args) && args[i+1] == "5" {
				st.fg, st.base = xterm256(args[i+2])
				i += 2
			}
		case "39":
			st.fg, st.base = tm.fg, -1
		}
	}
	return st
}

// --- colors ---

func xterm256(param string) (color.RGBA, int) {
	var n int
	fmt.Sscanf(param, "5;%d", &n)
	if n < 0 || n > 255 {
		return tm.fg, -1
	}
	if n < 16 {
		return tm.ansi[n], -1
	}
	if n < 232 {
		i := n - 16
		lv := [6]int{0, 95, 135, 175, 215, 255}
		return color.RGBA{uint8(lv[i/36]), uint8(lv[(i%36)/6]), uint8(lv[i%6]), 0xff}, -1
	}
	g := 8 + (n-232)*10
	return color.RGBA{uint8(g), uint8(g), uint8(g), 0xff}, -1
}

func hex(s string) color.RGBA {
	var r, g, b uint8
	if _, err := fmt.Sscanf(s, "#%02x%02x%02x", &r, &g, &b); err != nil {
		panic(err)
	}
	return color.RGBA{r, g, b, 0xff}
}

func die(err error) {
	if err != nil {
		panic(err)
	}
}
