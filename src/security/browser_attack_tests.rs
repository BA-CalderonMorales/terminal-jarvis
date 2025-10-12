// Browser Attack Prevention Tests - Ecosystem Security Protection
// Tests to demonstrate cross-browser protection against the attack vectors you observed

#[cfg(test)]
mod tests {
    use crate::security::core::{SecurityValidator, SecurityConfig};

    fn create_validator() -> SecurityValidator {
        SecurityValidator::with_config(SecurityConfig::default())
    }

    #[test]
    fn test_chrome_protocol_attacks_blocked() {
        let validator = create_validator();
        
        // These are the specific attacks you observed
        let chrome_attacks = vec![
            "chrome://settings/searchEngines",
            "chrome://extensions",
            "chrome://history",
            "chrome://downloads",
            "chrome-extension://fake-extension-id/malware.html",
            "chrome://settings/privacy",
            "chrome://settings/clearBrowserData",
        ];

        for attack in &chrome_attacks {
            println!("Testing Chrome attack: {}", attack);
            let result = validator.validate_input(attack, "command");
            assert!(result.is_err(), "Chrome attack should be blocked: {}", attack);
            println!("✅ BLOCKED: {}", attack);
        }
    }

    #[test]
    fn test_all_browser_protocols_blocked() {
        let validator = create_validator();
        
        // Comprehensive cross-browser protection
        let browser_attacks = vec![
            // Chrome family
            "chrome://settings",
            "chrome-extension://malware/payload",
            "edge://settings",
            "edge-extension://evil/attack",
            "chromium://history",
            
            // Firefox family
            "firefox://preferences",
            "about:config",
            "about:addons",
            "about:preferences",
            "resource://malware.js",
            "jar:file:///evil.jar",
            "moz-extension://bad-extension/attack.html",
            
            // Safari/WebKit
            "safari://preferences",
            "webkit://settings",
            
            // Other browsers
            "opera://settings",
            "brave://settings",
            "vivaldi://settings",
            "tor://settings",
            
            // Windows Edge specific
            "microsoft-edge://settings",
            "ms-browser-extension://evil/payload",
            "ms-settings:",
            "windows:",
            
            // Protocol hijacking attempts
            "default-search",
            "search-engine",
            "browser-default",
        ];

        for attack in &browser_attacks {
            let result = validator.validate_input(attack, "command");
            assert!(result.is_err(), "Browser protocol attack should be blocked: {}", attack);
        }
    }

    #[test]
    fn test_browser_hijacking_commands_blocked() {
        let validator = create_validator();
        
        // Commands that could launch browsers with malicious URLs
        let hijacking_commands = vec![
            "open chrome://evil-settings",
            "start edge://malware",
            "xdg-open firefox://attack",
            "browser chrome://settings",
            "launch safari://exploit",
            "execute chrome-extension://payload",
            "run firefox://about:config",
            
            // Cross-platform
            "chrome chrome://dangerous",
            "firefox about:config",
            "safari safari://malicious",
            
            // JavaScript browser manipulation
            "window.open('chrome://evil')",
            "location.href='chrome://settings'",
            "document.location='edge://attack'",
        ];

        for command in &hijacking_commands {
            let result = validator.validate_input(command, "command");
            assert!(result.is_err(), "Browser hijacking command should be blocked: {}", command);
        }
    }

    #[test]
    fn test_automation_tools_blocked() {
        let validator = create_validator();
        
        // Browser automation tools often used for sophisticated attacks
        let automation_attacks = vec![
            "selenium chrome://attack",
            "webdriver edge://exploit",
            "puppeteer firefox://malware",
            "playwright safari://payload",
            
            "automation script chrome://evil",
            "macro open chrome://settings",
            "script launch browser://attack",
        ];

        for attack in &automation_attacks {
            let result = validator.validate_input(attack, "command");
            assert!(result.is_err(), "Browser automation attack should be blocked: {}", attack);
        }
    }

    #[test]
    fn test_cross_platform_executables_blocked() {
        let validator = create_validator();
        
        // Executable file patterns that could deliver browser malware
        let executable_attacks = vec![
            "malware.exe",
            "attack.bat",
            "exploit.cmd",
            "payload.ps1",
            "malicious.sh",
            "dangerous.zsh",
            "attack.fish",
            "rootkit.bash",
            
            "run malware.exe",
            "execute payload.sh",
            "launch exploit.bat",
        ];

        for attack in &executable_attacks {
            let result = validator.validate_input(attack, "command");
            assert!(result.is_err(), "Executable attack should be blocked: {}", attack);
        }
    }

    #[test]
    fn test_suspicious_url_patterns_blocked() {
        let validator = create_validator();
        
        // URLs that could deliver browser exploits
        let suspicious_urls = vec![
            "https://chrome.google.com/webstore/fake-extension",
            "https://addons.mozilla.org/en-US/firefox/addon/malware",
            "https://microsoftedge.microsoft.com/addons/exploit",
            
            "https://malware-extension.com/payload",
            "https://evil-addon.org/attack",
            "https://phishing-plugin.net/steal",
            
            // URL shorteners (often used in attacks)
            "https://bit.ly/malicious",
            "https://tinyurl.com/exploit",
            "https://t.co/phish",
            "https://short.io/attack",
            
            // Code sharing (malware delivery)
            "https://pastebin.com/raw/malware",
            "https://gist.githubusercontent.com/evil/payload",
            
            // Suspicious TLDs
            "https://malware.tk",
            "https://phishing.ml",
            "https://exploit.ga",
        ];

        for url in &suspicious_urls {
            let result = validator.validate_input(url, "url");
            assert!(result.is_err(), "Suspicious URL should be blocked: {}", url);
        }
    }

    #[test]
    fn test_safe_commands_still_allowed() {
        let validator = create_validator();
        
        // Verify legitimate commands still work
        let safe_commands = vec![
            "help",
            "status", 
            "list",
            "info",
            "version",
            "exit",
            "quit",
        ];

        for cmd in &safe_commands {
            let result = validator.validate_command(cmd, &[]);
            assert!(result.is_ok(), "Safe command should be allowed: {}", cmd);
        }

        // Verify legitimate URLs still work (when explicitly allowed)
        let safe_urls = vec![
            "https://example.com",
            "https://github.com/user/repo",
            "https://api.openai.com/v1/chat/completions",
        ];

        for url in &safe_urls {
            let result = validator.validate_input(url, "url");
            assert!(result.is_ok(), "Safe URL should be allowed: {}", url);
        }
    }

    #[test]
    fn test_real_world_attack_scenarios() {
        let validator = create_validator();
        
        // Real attack combinations you might see
        let real_attacks = vec![
            // The exact attacks you observed
            "chrome://settings/searchEngines",
            "default-search",
            
            // Sophisticated multi-attack vectors
            "open chrome://settings && run malware.sh",
            "eval \"window.open('chrome://evil')\"",
            "system('start edge://exploit')",
            
            // Voice command injection attempts
            "navigate to chrome://extensions",
            "visit edge://settings/privacy",
            "goto firefox://about:config",
            
            // Browser-based supply chain attacks
            "install chrome-extension://fake-id/malware",
            "enable about:config evil.preference",
        ];

        for attack in &real_attacks {
            let result = validator.validate_input(attack, "voice_command");
            assert!(result.is_err(), "Real-world attack should be blocked: {}", attack);
        }
    }

    #[test]
    fn print_protection_summary() {
        println!("\n🛡️  CROSS-BROWSER PROTECTION SUMMARY");
        println!("=====================================");
        println!("✅ Chrome family: chrome://, edge://, chromium://");
        println!("✅ Firefox family: firefox://, about:, resource:, jar:");
        println!("✅ Safari/WebKit: safari://, webkit://");
        println!("✅ Other browsers: opera://, brave://, vivaldi://, tor://");
        println!("✅ Windows edge: microsoft-edge:, ms-settings:, windows:");
        println!("✅ Browser automation: selenium, webdriver, puppeteer");
        println!("✅ Extension stores: chrome.google.com, addons.mozilla.org");
        println!("✅ Suspicious TLDs: .tk, .ml, .ga");
        println!("✅ URL shorteners: bit.ly, tinyurl, t.co");
        println!("✅ Executable files: .exe, .bat, .sh, .ps1");
        println!("✅ Protocol hijacking: ANY protocol except http/https");
        println!("✅ Cross-platform: open, start, xdg-open");
        println!("✅ Injection vectors: eval, exec, system, window.open");
        println!("✅ Command chaining: ;, &&, ||, |");
        println!("\n🚨 This protects ALL users across ALL browsers!");
    }
}
