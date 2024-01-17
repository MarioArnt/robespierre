manifest=$(jq -r '.version' ./npm/robespierre/package.json)
npm=$(npm view robespierre --json | jq -r '.version')
[[ "$manifest" == "$npm" ]] && echo "false" || echo "true"
