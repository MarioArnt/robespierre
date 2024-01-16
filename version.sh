release=$1
sed "s/\"version\": \"[0-9\.]+\"/\"version\": \"${release}\"/" ./npm/robespierre/package.json
sed "s/\"robespierre-([a-z0-9-]+)\": \"[0-9\.]+\"/\"robespierre-\1\": \"${release}\"/g" ./npm/robespierre/package.json
sed "s/^version = \"[0-9\.]+\"$/version = \"${release}\"/" ./Cargo.toml
cat ./npm/robespierre/package.json
cat ./Cargo.toml
