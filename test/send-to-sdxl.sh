# 直接发送 text2img 到 SDXL 上，越过hoopics服务
curl -X POST -H "Content-Type: application/json" -d '{"prompt":"一个丰富多彩","job_id":"job_123","style":"normal","model_type":"SDXL","width":1024,"height":1024}' https://u447140-b619-b81b9121.bjb1.seetacloud.com:8443/txt2img | jq .
