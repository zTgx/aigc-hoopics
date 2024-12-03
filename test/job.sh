curl -X POST http://localhost:3030/v1/job -H "Authorization: Bearer valid_token" -H "Content-Type: application/json" -d '{"job": "北京"}' | jq .
