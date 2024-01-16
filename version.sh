release=$1
sed -Ei '' "s/\"version\": \"[0-9\.]+\"/\"version\": \"${release}\"/" ./npm/robespierre/package.json
sed -Ei '' "s/\"robespierre-([a-z0-9-]+)\": \"[0-9\.]+\"/\"robespierre-\1\": \"${release}\"/g" ./npm/robespierre/package.json
sed -Ei '' "s/^version = \"[0-9\.]+\"$/version = \"${release}\"/" ./Cargo.toml
cat ./npm/robespierre/package.json
cat ./Cargo.toml
