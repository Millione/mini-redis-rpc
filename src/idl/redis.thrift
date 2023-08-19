namespace rs redis

struct GetResp {
    1: string value,
}

struct SetReq {
    1: required string key,
    2: required string value,
    3: i64 expires,
}

service RedisService {
    GetResp get(1: string key),
    void set(1: SetReq req),
    bool del(1: string key),
    void ping(),
}
