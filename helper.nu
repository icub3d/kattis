#!/usr/bin/env nu

source $nu.config-path

# Download the sample inputs from Kattis for a given problem
def get-input [name: string, part="1": string] {
  let input_dir = ("inputs" | path join $name)
  mkdir $input_dir
  let zip_path = ($input_dir | path join "samples.zip")

  if ($zip_path | path exists) {
    print $"Samples for ($name) already downloaded. Skipping."
    return
  }

  let url = $"https://open.kattis.com/problems/($name)/file/statement/samples.zip"
  print $"🚀 Downloading samples for ($name) from ($url)..."

  try {
    let response = http get $url
    $response | save --force $zip_path
    print $"✅ Successfully saved samples to '($zip_path)'"
    cd $input_dir
    unzip samples.zip
  } catch { |error|
    print $"❌ Failed to download samples."
    print $"Reason: ($error.msg)"
  }
}

# Show the problem description in the terminal
def show [name: string] {
  let url = $"https://open.kattis.com/problems/($name)"
  let lines_list = (http get $url | 
    pup .problembody --charset UTF-8 | 
    sd '^\s+' '' |
    w3m -T text/html -dump -cols 10000 |
    lines)
  
  mut img_counter = 1
  for $l in $lines_list {
    if ($l | str starts-with '/') { 
      try { http get $"https://open.kattis.com($l)" | ^viu - } catch { }
    } else if ($l | str contains '\includegraphics') {
      let img_num = ($img_counter | fill -a right -c '0' -w 4)
      try { http get $"https://open.kattis.com/problems/($name)/file/statement/en/img-($img_num).png" | ^viu - } catch { }
      $img_counter = $img_counter + 1
    } else { 
      print $l 
    }
  }
}

# Submit a solution to Kattis
def submit [name: string] {
  let config_path = ".kattis.ini"
  if not ($config_path | path exists) {
    print $"❌ no .kattis.ini found. Go to https://open.kattis.com/info/submit"
    return
  }

  let config = (parse ini $config_path)
  let language = 'Rust'
  let file_path = ("src" | path join "bin" $"($name).rs")

  if not ($file_path | path exists) {
      print $"❌ Solution file not found: ($file_path)"
      return
  }

  # 1. Login and get cookies
  print -n "Status: Logging in..."
  let login_response = http post -f --content-type "application/x-www-form-urlencoded" --headers ['User-Agent', 'kattis-cli-submit'] ($config | get kattis.loginurl) {
    user: ($config | get user.username),
    token: ($config | get user.token),
    script: "true"
  }

  if $login_response.status != 200 {
      print $"❌ Login failed."
      return
  }

  let cookies = ($login_response.headers.response
    | where name == "set-cookie"
    | get value
    | each { |s| $s | split row ";" | first }
    | str join "; ")

  # 2. Prepare submission data and submit
  print -n $"\r(ansi erase_line)Status: Submitting..."
  
  let submit_url = ($config | get kattis.submissionurl)
  let file_basename = ($file_path | path basename)
  
  let submit_output = (do -i {
    ^curl -sS -H "User-Agent: kattis-cli-submit" -H $"Cookie: ($cookies)" -F "submit=true" -F "submit_ctr=2" -F $"language=($language)" -F $"problem=($name)" -F "script=true" -F $"mainclass=($file_basename)" -F $"sub_file[]=@($file_path)" $submit_url
  } | complete)

  if $submit_output.exit_code != 0 {
      print $"❌ Submission failed."
      print $submit_output.stderr
      return
  }

  let response_text = $submit_output.stdout
  # Parse the submission id robustly (HTML or plain text)
  let submission_id = do {
    let found_lines = ($response_text | lines | find -r '^\s*Submission ID:\s*')
    let by_line = if ($found_lines | is-empty) { "" } else { 
      $found_lines | first | str replace -r '^\s*Submission ID:\s*' "" | str trim 
    }
    if not ($by_line | is-empty) { $by_line } else {
      let parsed = ($response_text | parse -r 'Submission ID:\s*(?<id>\d+)')
      if ($parsed | is-empty) { "" } else { $parsed | get id | first }
    }
  }

  if ($submission_id | is-empty) {
      print "" # newline
      print $"❌ Could not get submission ID from response:"
      print $response_text
      return
  }

  let submission_url = $"(($config | get kattis.submissionsurl))/($submission_id)"

  # 3. Check status
  let get_status_text = {|id|
    match $id {
      0 | 1 => 'New',
      2 => 'Waiting for compile',
      3 => 'Compiling',
      4 => 'Waiting for run',
      5 => 'Running',
      6 => 'Judge Error',
      8 => 'Compile Error',
      9 => 'Run Time Error',
      10 => 'Memory Limit Exceeded',
      11 => 'Output Limit Exceeded',
      12 => 'Time Limit Exceeded',
      13 => 'Illegal Function',
      14 => 'Wrong Answer',
      16 => 'Accepted',
      _ => $"Unknown status ($id)"
    }
  }

  while true {
    sleep 1sec
    let status_url = $"($submission_url)?json"
    let status_data = (http get --headers [
        "User-Agent", "kattis-cli-submit",
        "Cookie", $cookies
      ] $status_url)

    let status_id = ($status_data.status_id | into int)
    let status_text = (do $get_status_text $status_id)

    if $status_id < 5 { # Not running yet
        print -n $"\r(ansi erase_line)Status: ($status_text)"
    } else {
        let testcases_done = $status_data.testcase_index
        let row_html = ($status_data.row_html | default "")
        # Try to count testcases; fall back gracefully if tools are unavailable
        let testcases_total = (try { 
          let count_str = ($row_html | ^pup 'i json{}' | from json | length)
          $count_str - 1
        } catch { 0 })
        if $testcases_total > 0 {
          print -n $"\r(ansi erase_line)Status: ($status_text) [($testcases_done)/($testcases_total)]"
        } else {
          print -n $"\r(ansi erase_line)Status: ($status_text) [($testcases_done)]"
        }
    }

    if $status_id > 5 { # Finished
        print "" # Newline after final status
        if $status_id == 16 { # Accepted
            print $"✅ ($status_text)"
        } else {
            print $"❌ ($status_text)"
        }
        let feedback_html = (try { $status_data.feedback_html } catch { "" })
        if not ($feedback_html | is-empty) {
            let feedback_text = (try { $feedback_html | ^pup 'pre' text{} | str trim } catch { $feedback_html })
            print $feedback_text
        }
        break
    }
  }
}

# Test runner for Kattis problems
def run-tests [name: string, --quiet (-q), --relative-error (-r): number] {
    print $"🐱 Kattis ($name) 🐱"

    let input_dir = ("inputs" | path join $name)

    if not ($input_dir | path exists) {
        print $"❌ Input directory not found: ($input_dir)"
        return
    }

    let input_files = (ls $input_dir | where name =~ '\.in$' | get name)

    let build_result = (cargo build --release -q --bin $name | complete)

    if $build_result.exit_code != 0 {
        print $"❌ Build failed"
        if not ($build_result.stderr | is-empty) {
            print $build_result.stderr
        }
        return
    }

    let binary_name = if $nu.os-info.name == "windows" { $"($name).exe" } else { $name }
    let binary_path = ("target" | path join "release" $binary_name)

    if not ($binary_path | path exists) {
        print $"❌ Binary not found at ($binary_path)"
        return
    }

    # If no input files, just run the binary once
    if ($input_files | is-empty) {
        print $"⚠️  No .in files found, running binary without input..."
        let timing = (timeit { ^$binary_path | complete })
        let duration = $timing
        let output = (^$binary_path | complete)
        
        if $output.exit_code != 0 {
            print $"❌ Program failed with exit code ($output.exit_code)"
            if not ($output.stderr | is-empty) {
                print $output.stderr
            }
        } else {
            print $"📝 manual run ✅ \(($duration))"
            print $output.stdout
        }
        return
    }

    for $input_file in $input_files {
        let ans_file = ($input_file | str replace '.in' '.ans')
        let test_name = ($input_file | path basename)

        if not ($ans_file | path exists) {
            print $"❌ Answer file not found: ($ans_file)"
            continue
        }

        let timing = (timeit { cat $input_file | ^$binary_path | complete })
        let duration = $timing

        let output = (cat $input_file | ^$binary_path | complete)

        if $output.exit_code != 0 {
            print $"❌ Program failed with exit code ($output.exit_code)"
            if not ($output.stderr | is-empty) {
                print $output.stderr
            }
            continue
        }

        let expected = (cat $ans_file | str trim)
        let actual = ($output.stdout | str trim)

        if $relative_error != null {
            try {
                let exp = ($expected | into float)
                let act = ($actual | into float)
                let abs_diff = (($act - $exp) | math abs)
                let denom = ($exp | math abs)
                let rel = if $denom != 0 { $abs_diff / $denom } else { $abs_diff }
                if $rel <= $relative_error {
                    print $"📝 ($test_name) ✅ \(($duration))"
                } else {
                    print $"📝 ($test_name) ❌ \(($duration))"
                    if not $quiet {
                        print $"Expected:\n($expected)"
                        print $"Got:\n($actual)"
                        print $"Variance: ($rel)"
                    } else {
                        break
                    }
                    return
                }
            } catch { |err|
                print $"❌ Failed to compare as floats: ($err.msg)"
            }
        } else {
            if $expected == $actual {
                print $"📝 ($test_name) ✅ \(($duration))"
            } else {
                print $"📝 ($test_name) ❌ \(($duration))"
                if not $quiet {
                    print $"Expected:\n($expected)"
                    print $"Got:\n($actual)"
                } else {
                    break
                }
                return
            }
        }
    }
}

# Helper for watch command
def watch-helper [name: string, --relative-error (-r): number] {
    clear
    try { 
        run-tests $name --relative-error=$relative_error
    } catch { |err| 
        print $"❌ Compilation failed: ($err.msg)"
        print "🔄 Watching for changes..."
    }
}

# Watch for changes and run tests
def watch-cmd [name: string, --relative-error (-r): number] {
  watch-helper $name --relative-error=$relative_error
  watch --quiet . --glob=**/*.rs {||
    watch-helper $name --relative-error=$relative_error
  }
}

# Upload solution to GitHub Gist
def gist [name: string] {
  let file_path = ("src" | path join "bin" $"($name).rs")

  if not ($file_path | path exists) {
    print $"❌ Solution file not found: ($file_path)"
    return
  }

  print $"🚀 Uploading ($file_path) to GitHub Gist..."
  let result = (gh gist create $file_path --desc $"Kattis ($name)" --public | complete)
  
  if $result.exit_code == 0 {
    print $"✅ Gist uploaded successfully!"
    print $result.stdout
  } else {
    print $"❌ Failed to upload Gist."
    if not ($result.stderr | is-empty) {
      print $result.stderr
    }
  }
}

# Generate YouTube title and description for a Kattis problem
def yt [name: string] {
  let file_path = ("src" | path join "bin" $"($name).rs")

  if not ($file_path | path exists) {
    print $"❌ Solution file not found: ($file_path)"
    return
  }

    # Create new gist
    print $"🚀 Creating GitHub Gist ($file_path) ..."
    let result = (gh gist create $file_path --desc $"Kattis ($name)" --public | complete)
    
  let gist_url = if $result.exit_code == 0 {
      print $"✅ Gist created successfully!"
      $result.stdout | str trim
    } else {
      print $"❌ Failed to create Gist."
      if not ($result.stderr | is-empty) {
        print $result.stderr
      }
      return
  }

  # Generate title and description
  print $"($name) - Daily Kattis #coding #codeprep #programming #codingchallenge"
  print ""
  print $"🚀 Solving ($name) today! Solve one with me daily to stay sharp! 💪"
  print ""
  print $"Problem: https://open.kattis.com/problems/($name)"
  print $"Solution: ($gist_url)"
}

# Get the expected output for a given problem and sample number
def get-target [name: string, part="1": string] {
  let input_dir = ("inputs" | path join $name)
  
  if not ($input_dir | path exists) {
    print-error $"Input directory not found: ($input_dir)"
    return ""
  }

  # Try different naming patterns
  let patterns = [
    $"($name).0($part).ans",
    $"($part).ans"
  ]

  for $pattern in $patterns {
    let file_path = ($input_dir | path join $pattern)
    if ($file_path | path exists) {
      return (open $file_path | str trim)
    }
  }

  print-error $"No answer file found for problem '($name)' part '($part)'"
  return ""
}

def main [command?: string, ...args] {
  if ($command | is-empty) {
    print "Usage: helper.nu <command> [args...]"
    print "Available commands:"
    print "  get-input <name> [part]"
    print "  get-target <name> [part]"
    print "  open <name>"
    print "  show <name>"
    print "  submit <name>"
    print "  gist <name>"
    print "  yt <name>"
    print "  watch <name> [--relative-error <number>]"
    return
  }

  let func_name = $command | str replace "-" "_"
  
  match $command {
    "get-input" => {
      if ($args | length) < 1 {
        print "Usage: get-input <name> [part]"
        return
      }
      let name = ($args | get 0)
      let part = (if ($args | length) >= 2 { $args | get 1 } else { "1" })
      get-input $name $part
    }
    "get-target" => {
      if ($args | length) < 1 {
        print "Usage: get-target <name> [part]"
        return
      }
      let name = ($args | get 0)
      let part = (if ($args | length) >= 2 { $args | get 1 } else { "1" })
      get-target $name $part
    }
    "open" => {
      if ($args | length) < 1 {
        print "Usage: open <name>"
        return
      }
      let name = ($args | get 0)
      open $name
    }
    "show" => {
      if ($args | length) < 1 {
        print "Usage: show <name>"
        return
      }
      let name = ($args | get 0)
      show $name
    }
    "submit" => {
      if ($args | length) < 1 {
        print "Usage: submit <name>"
        return
      }
      let name = ($args | get 0)
      submit $name
    }
    "yt" => {
      if ($args | length) < 1 {
        print "Usage: yt <name>"
        return
      }
      let name = ($args | get 0)
      yt $name
    }
    "gist" => {
      if ($args | length) < 1 {
        print "Usage: gist <name>"
        return
      }
      let name = ($args | get 0)
      gist $name
    }
    "watch" => {
      if ($args | length) < 1 {
        print "Usage: watch <name> [--relative-error <number>]"
        return
      }
      let name = ($args | get 0)
      # Check if there's a --relative-error flag
      let has_rel_error = ($args | any {|x| $x == "--relative-error" or $x == "-r"})
      if $has_rel_error {
        let idx = ($args | enumerate | where item == "--relative-error" or item == "-r" | get index | first)
        let rel_error = ($args | get ($idx + 1) | into float)
        watch-cmd $name --relative-error=$rel_error
      } else {
        watch-cmd $name
      }
    }
    _ => {
      print $"Unknown command: ($command)"
      print "Available commands:"
      print "  get-input <name> [part]"
      print "  get-target <name> [part]"
      print "  open <name>"
      print "  show <name>"
      print "  submit <name>"
      print "  gist <name>"
      print "  yt <name>"
      print "  watch <name> [--relative-error <number>]"
    }
  }
}

