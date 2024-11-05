# justfile for just.systems recipes. 

version := `cat Cargo.toml | grep version | head -n 1 | cut -d '"' -f 2`

# 1. Create a new tag to trigger a release.
# 2. Push the tag to GitHub.
# 3. GitHub Actions will then build and publish the release.
release-tag:
  echo "Releasing version {{version}}"
  git tag -a v{{version}} -m "Release version {{version}}"
  git push origin v{{version}}


