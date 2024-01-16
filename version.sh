release=$1
echo "Updating files for $release release."
sed -Ei '' "s/\"version\": \"([0-9a-zA-Z]|-|\.)+\"/\"version\": \"${release}\"/" ./npm/robespierre/package.json
sed -Ei '' "s/\"robespierre-(.+)\": \"([0-9a-zA-Z]|-|\.)+\"/\"robespierre-\1\": \"${release}\"/g" ./npm/robespierre/package.json
sed -Ei '' "s/^version = \"([0-9a-zA-Z]|-|\.)+\"$/version = \"${release}\"/" ./Cargo.toml
cat ./npm/robespierre/package.json
cat ./Cargo.toml
cargo build
# shellcheck disable=SC2164
cd ./npm/robespierre
npm i
