# zms-tradition-data
zms-tradition-data 

## 组件

 clickhouse
 redis

## 设计思路
 1. 维护交易所(币安)历史数据[历史最后维护时间到现在,历史最远维护时间到过去;历史维护时间不存在就取当前时间到现在]
 2. 以ticker数据订阅事件更新最新k线数据 todo;弥补定时任务延时情况
 3. 提供数据接口 时间范围 周期 币种名称,redis缓存机制 
## 数据监听 统计
 1. 初筛币种，板块，市值，交易量流动性评估 直接第三方网站爬,保存到redis
 2. SOL链DEX交易实时监控，链上事件数据过滤； 
 3. 聪明钱包，dev统计，跟踪
 4. 机器人更敏感的数据监听狙击，跟单，套利
## 
https://coinmarketcap.com/api/documentation/v1/#section/Authentication