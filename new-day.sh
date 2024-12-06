#!/bin/zsh

DAY=$1
RAW=$2

if [[ -d "day$DAY" ]]; then
  echo "Day $DAY already exists"
  exit 1
fi

mkdir -p "day$DAY"
mkdir -p "day$DAY/src"

INPUT_IMPORT="std::fs::read_to_string"
UTILS_IMPORT="write_day_input_to_file"
INPUT_TYPE="String"
INPUT_DEF="let input = read_to_string(input_path)?;"

if [[ -z $RAW ]]; then
  INPUT_IMPORT="itertools::Itertools"
  UTILS_IMPORT="{read_lines, write_day_input_to_file}"
  INPUT_TYPE="Vec<String>"
  INPUT_DEF="let input = read_lines(input_path)?
                     .collect::<Result<Vec<String>, io::Error>>()?;"
fi

echo "use std::env;
use $INPUT_IMPORT;
use std::io::{self};
use utils::$UTILS_IMPORT;

fn main() -> io::Result<()> {
    let session_cookie = env::var(\"AOC_SESSION_COOKIE\").unwrap_or_else(|_| \"0\".to_string());

    // Query day $DAY for input
    let input_path = write_day_input_to_file(\"$DAY\", session_cookie.as_str()).unwrap();
    $INPUT_DEF

    // Part 1
    println!(\"Part 1: {}\", part_one(&input));
    // Part 2
    println!(\"Part 2: {}\", part_two(&input));

    Ok(())
}

fn part_one(input: &$INPUT_TYPE) -> int64 {
  let mut total_sum = 0;
  total_sum
}

fn part_two(input: &$INPUT_TYPE) -> int64 {
  let mut total_sum = 0;
  total_sum
}

const TEST_INPUT: &str = \"\
\";

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_day_one() {
    let expected = 0;
    assert_eq!(expected, part_one(&TEST_INPUT.to_string()));
  }

  #[test]
    fn test_day_two() {
      let expected = 0;
      assert_eq!(expected, part_two(&TEST_INPUT.to_string()));
    }
}
" > "day$DAY/src/main.rs"

echo "[package]
name = \"day$DAY\"
version = \"0.1.0\"
authors = [\"tagptroll1\"]
edition = \"2021\"

[dependencies]
\"utils\" = { path = \"../utils\" }
" > "day$DAY/Cargo.toml"

sed -i '' -e ":a" -e "\$!N;/\n[[:space:]]*]/!ba" -e "s/\n[[:space:]]*]/\n    \"day$DAY\",\n]/" "Cargo.toml"

cargo fmt -- "day$DAY/src/main.rs"

AOC_SESSION_COOKIE=$(<cookie.txt)
CONFIG_PATH="../.idea/runConfigurations/Run_Day$DAY.xml"

echo "<component name=\"ProjectRunConfigurationManager\">
        <configuration default=\"false\" name=\"Run day$DAY\" type=\"CargoCommandRunConfiguration\" factoryName=\"Cargo Command\">
          <option name=\"buildProfileId\" value=\"dev\" />
          <option name=\"command\" value=\"run --package day$DAY --bin day$DAY\" />
          <option name=\"workingDirectory\" value=\"file://\$PROJECT_DIR\$/aoc2024\" />
          <envs>
            <env name=\"AOC_SESSION_COOKIE\" value=\"$AOC_SESSION_COOKIE\" />
          </envs>
          <option name=\"emulateTerminal\" value=\"true\" />
          <option name=\"channel\" value=\"DEFAULT\" />
          <option name=\"requiredFeatures\" value=\"true\" />
          <option name=\"allFeatures\" value=\"false\" />
          <option name=\"withSudo\" value=\"false\" />
          <option name=\"buildTarget\" value=\"REMOTE\" />
          <option name=\"backtrace\" value=\"SHORT\" />
          <option name=\"isRedirectInput\" value=\"false\" />
          <option name=\"redirectInputPath\" value=\"\" />
          <method v=\"2\">
            <option name=\"CARGO.BUILD_TASK_PROVIDER\" enabled=\"true\" />
          </method>
        </configuration>
      </component>
" > "$CONFIG_PATH"