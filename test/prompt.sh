
curl -X POST http://localhost:3030/v1/prompt -H "Authorization: Bearer valid_token" -H "Content-Type: application/json" -d '{"prompt": "北京"}' | jq .