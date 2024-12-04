curl -X POST http://localhost:3030/v1/job-status -H "Authorization: Bearer valid_token" -H "Content-Type: application/json" -d '{"job_ids": ["cf628bc0-c15f-4966-aab6-f0c3e8bd2b57", "acf2e880-de0b-4052-807d-dbc14f4ff52b"]}'  | jq .

# [
#   {
#     "file_urls": [
#       "https://nft1000.oss-cn-beijing.aliyuncs.com/sd_output/txt2img/2024-12-04/t2i-2024-12-04-09-44-13-cf628bc0-c15f-4966-aab6-f0c3e8bd2b57-0.png"
#     ],
#     "job_id": "cf628bc0-c15f-4966-aab6-f0c3e8bd2b57",
#     "status": "success"
#   },
#   {
#     "file_urls": [
#       "https://nft1000.oss-cn-beijing.aliyuncs.com/sd_output/txt2img/2024-12-04/t2i-2024-12-04-09-45-06-acf2e880-de0b-4052-807d-dbc14f4ff52b-0.png"
#     ],
#     "job_id": "acf2e880-de0b-4052-807d-dbc14f4ff52b",
#     "status": "success"
#   }
# ]
