SCRIPT_PATH=$(dirname $(realpath -s $0))
rm $SCRIPT_PATH/../*.yml
rm $SCRIPT_PATH/../*.yml.bk

echo "Should get Key not found"
curl -X GET "http://localhost:3000/foo" 
echo ""
echo ""

echo "Should be OK"
curl -X PUT "http://localhost:3000/foo" -d "{\"hi\": \"there\"}"
echo ""
echo ""

echo "Should get some value"
curl -X GET "http://localhost:3000/foo" 
echo ""
echo ""

echo "Should be OK"
curl -X DELETE "http://localhost:3000/foo"
echo ""
echo ""

echo "Should get Key not found"
curl -X GET "http://localhost:3000/foo" 
echo ""
echo ""

