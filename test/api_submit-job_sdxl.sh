curl -X POST http://localhost:3030/v1/submit-job \
-H "Authorization: Bearer valid_token" \
-H "Content-Type: application/json" \
-d '{
    "prompt": "来一个SDXL",
    "negative_prompt": "",
    "job_type": 0,
    "priority": 1,
    "description": "",
    "job_style": 0,
    "model": 0,
    "width": 1024,
    "height": 1024,
    "rewrite_prompt": true
}'