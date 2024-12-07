# 直接查询 FLUX job status, 越过 hoopics 服务
curl -X POST -H "Content-Type: application/json" -d '{"job_ids":["ff8ba8c7-ec26-4b1c-aa89-50d23a9acb63", "487f9a9f-8d62-412a-89cb-a1b2df309b36", "7329130a-3451-46fc-976b-90eaf13a42a4"]}' https://u447140-9ec2-9aec6315.bjb1.seetacloud.com:8443/get_task_status_batch | jq .

# 返回数据：
# [
#    {
#       "file_urls":["https://nft1000.oss-cn-beijing.aliyuncs.com/sd_output/txt2img/2024-12-04/t2i-2024-12-04-07-17-20-job_123-0.png"],
#       "job_id":"job_123",
#       "status":"success"
#    }
#]
