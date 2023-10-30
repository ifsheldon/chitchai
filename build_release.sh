rm -r docs
dx build --release
cp -r dist docs
echo "chitchai.dev" > docs/CNAME