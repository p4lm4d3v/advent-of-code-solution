param (
  [Parameter(Mandatory = $true)]
  [int]$year,
  [Parameter(Mandatory = $true)]
  [int]$day
)

# Appends the list of strings to the end of the file
function Special-Write {
    param (
        [string]$path,
        [string[]]$lines
    )

    foreach ($line in $lines) {
        $line | Out-File -FilePath $path -Append -Encoding utf8
    }
} 

function Create-Part { 
    param (
      [int]$n
    )

    New-Item -ItemType Directory -Path part$n      *>&1 | Out-Null 
    Set-Location part$n

    New-Item -ItemType File      -Path input$n.txt *>&1 | Out-Null
    "" | Out-File -FilePath input$n.txt -Append -Encoding utf8
    New-Item -ItemType File      -Path test$n.rs   *>&1 | Out-Null 
    Special-Write -path test$n.rs -lines @(
      "#[cfg(test)]",
      "mod tests$n {",
      "  use crate::process;",
      "}"
    )
    New-Item -ItemType File -Path part$n.rs *>&1 | Out-Null 
    Special-Write -path part$n.rs -lines @(
      "mod tests$n;",
      "",
      "fn main() {",
      "  let input = include_str!(`"./input$n.txt`");",
      "  let output = process(input);",
      "  dbg!(`"{}`", output);",
      "}",
      "",
      "fn process(input: &str) -> i32 {",
      "  todo!()",
      "}"
    )
    Set-Location ..
  }


# Directory names
$yearDir = "y$year"
$dayDir = "d$day"

# Check if the year directory exists
if ( -Not (Test-Path -Path $yearDir)) {
  # Create the year directory 
  New-Item -ItemType Directory -Path $yearDir *>&1 | Out-Null
}

Set-Location -Path $yearDir

if ( -Not (Test-Path -Path $dayDir)) {
  cargo new --bin $dayDir *>&1 | Out-Null

  Set-Location $dayDir
  "[package]" | Out-File -FilePath Cargo.toml -Encoding utf8
  Special-Write -path Cargo.toml @(
    "name = `"$dayDir`"",
    "version = `"0.1.0`"",
    "dition = `"2021`"",
    "",
    "[[bin]]",
    "name = `"part1`"",
    "path = `"src/part1/part1.rs`"",
    "",
    "[[bin]]"
    "name = `"part2`"",
    "path = `"src/part2/part2.rs`"",
    "",
    "[dependencies]"
  )


  Set-Location src

  Remove-Item main.rs
  Create-Part -n 1
  Create-Part -n 2

  Set-Location ..
  Set-Location ..
}

Set-Location ..
