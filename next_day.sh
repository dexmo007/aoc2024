set -e
set -e pipefail

most_recent_day=$(ls -d src/*/ | sed -rn 's/^src\/day(.*)\/$/\1/p' | xargs -n1 | sort -r | head -1)

new_day=$((10#$most_recent_day + 1))
new_day=$(printf "%02d" $new_day)

mkdir -p src/day$new_day
touch src/day$new_day/sample.txt
touch src/day$new_day/input.txt
cat <<EOF > src/day$new_day/mod.rs

use crate::aoc_core::{AocResult, AocTask};

pub struct Day$new_day;

impl AocTask for Day$new_day {

    fn solve_a(&self, contents: String) -> AocResult {
        todo!()
    }

    fn solve_b(&self, _contents: String) -> AocResult {
        todo!()
    }

}

EOF