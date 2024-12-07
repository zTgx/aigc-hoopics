# 直接查询 SDXL job status, 越过 hoopics 服务
curl -X POST -H "Content-Type: application/json" -d '{"job_ids":["a123a974-a6a3-4e0f-883a-41fcaf41a93c"]}' https://u447140-b619-b81b9121.bjb1.seetacloud.com:8443/get_task_status_batch | jq .

# 返回数据：
# [
#    {
#       "file_urls":["https://nft1000.oss-cn-beijing.aliyuncs.com/sd_output/txt2img/2024-12-04/t2i-2024-12-04-07-17-20-job_123-0.png"],
#       "job_id":"job_123",
#       "status":"success"
#    }
#]
