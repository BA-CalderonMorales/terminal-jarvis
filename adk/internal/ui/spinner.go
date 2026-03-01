package ui

import (
	"fmt"
	"sync"
	"time"
)

var startupMsgs = []string{
	"Waking up the brain",
	"Brewing digital coffee",
	"Untangling the wires",
	"Feeding the hamsters",
	"Polishing the gears",
	"Teaching it manners",
	"Rounding up the bits",
	"Asking nicely",
	"Charging the batteries",
	"Summoning the robots",
	"Warming up the neurons",
	"Loading awesomeness",
}

var spinnerFrames = []string{"   \u250c( >_<)\u2518", "   \u2514( >_<)\u2510"}

// Spinner runs an animated terminal spinner in a background goroutine.
// Stop it by closing the returned stop channel.
type Spinner struct {
	stop chan struct{}
	done chan struct{}
	mu   sync.Mutex
}

// StartSpinner launches the startup spinner (cycling messages).
func StartSpinner() *Spinner {
	s := &Spinner{stop: make(chan struct{}), done: make(chan struct{})}
	go func() {
		defer close(s.done)
		i := 0
		tick := time.NewTicker(350 * time.Millisecond)
		defer tick.Stop()
		for {
			frame := spinnerFrames[i%2]
			msg := startupMsgs[(i/6)%len(startupMsgs)]
			fmt.Printf("\r%s%s%s  %s%s...%s", Cyan, frame, Reset, Dim, msg, Reset)
			i++
			select {
			case <-s.stop:
				fmt.Printf("\r%-80s\r", "")
				return
			case <-tick.C:
			}
		}
	}()
	return s
}

// StartThinkingSpinner launches the "T.J thinking" spinner used during LLM calls.
func StartThinkingSpinner() *Spinner {
	s := &Spinner{stop: make(chan struct{}), done: make(chan struct{})}
	go func() {
		defer close(s.done)
		i := 0
		tick := time.NewTicker(350 * time.Millisecond)
		defer tick.Stop()
		for {
			frame := spinnerFrames[i%2]
			fmt.Printf("\r%s%s%s  ", Cyan, frame, Reset)
			i++
			select {
			case <-s.stop:
				fmt.Printf("\r%-60s\r", "")
				return
			case <-tick.C:
			}
		}
	}()
	return s
}

// Stop halts the spinner and erases its line. Safe to call multiple times.
func (s *Spinner) Stop() {
	s.mu.Lock()
	defer s.mu.Unlock()
	select {
	case <-s.stop:
		// already stopped
	default:
		close(s.stop)
	}
	<-s.done
}
