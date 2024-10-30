rm -r docs
rm -r dist
dx build --release
cp -r dist docs
echo "chitchai.reify.ing" > docs/CNAME
touch docs/.nojekyll
cp docs/index.html docs/404.html
echo "Done!"