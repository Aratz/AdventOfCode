year=$1
rustup update

mkdir -p $year/{data,rs}
cd $year/rs
cargo init --name Advent_of_Code_$year
rm src/main.rs
mkdir src/bin
