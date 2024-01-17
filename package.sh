cd npm
# check if already published
manifest=$(jq -r '.version' ./npm/robespierre/package.json)
npm=$(npm view robespierre --json | jq -r '.version')
[[ "$manifest" == "$npm" ]] && exit 0 || echo "publishing new version"
# set the binary name
bin="robespierre"
# derive the OS and architecture from the build matrix name
# note: when split by a hyphen, first part is the OS and the second is the architecture
node_os=$(echo "${BUILD_NAME}" | cut -d '-' -f1)
export node_os
node_arch=$(echo "${BUILD_NAME}" | cut -d '-' -f2)
export node_arch
# set the version
export node_version="${BUILD_VERSION}"
# set the package name
# note: use 'windows' as OS name instead of 'win32'
if [ "${BUILD_OS}" = "windows-2022" ]; then
  export node_pkg="${bin}-windows-${node_arch}"
else
  export node_pkg="${bin}-${node_os}-${node_arch}"
fi
# create the package directory
mkdir -p "${node_pkg}/bin"
# generate package.json from the template
envsubst < package.json.tmpl > "${node_pkg}/package.json"
# copy the binary into the package
# note: windows binaries has '.exe' extension
if [ "${BUILD_OS}" = "windows-2022" ]; then
  bin="${bin}.exe"
fi
cp "../target/${BUILD_TARGET}/release/${bin}" "${node_pkg}/bin"
# publish the package
if [ "$1" = "--publish" ]; then
  cp "../README.md" "${node_pkg}"
  cd "${node_pkg}"
  npm publish --access public
fi
