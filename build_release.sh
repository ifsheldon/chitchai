rm -r docs
rm -r dist
dx build --release
cp -r dist docs
echo "chitchai.dev" > docs/CNAME