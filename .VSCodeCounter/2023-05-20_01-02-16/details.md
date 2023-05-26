# Details

Date : 2023-05-20 01:02:16

Directory /home/jcfun/learning/Project/e-tech-assist/web-server

Total : 81 files,  5997 codes, 251 comments, 612 blanks, all 6860 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [web-server/Cargo.toml](/web-server/Cargo.toml) | TOML | 100 | 0 | 26 | 126 |
| [web-server/src/common/banner.rs](/web-server/src/common/banner.rs) | Rust | 12 | 0 | 2 | 14 |
| [web-server/src/common/errors.rs](/web-server/src/common/errors.rs) | Rust | 189 | 9 | 16 | 214 |
| [web-server/src/common/mod.rs](/web-server/src/common/mod.rs) | Rust | 3 | 0 | 1 | 4 |
| [web-server/src/common/res.rs](/web-server/src/common/res.rs) | Rust | 170 | 21 | 22 | 213 |
| [web-server/src/config/cfg.rs](/web-server/src/config/cfg.rs) | Rust | 88 | 29 | 12 | 129 |
| [web-server/src/config/ctx.rs](/web-server/src/config/ctx.rs) | Rust | 21 | 0 | 5 | 26 |
| [web-server/src/config/init.rs](/web-server/src/config/init.rs) | Rust | 14 | 2 | 6 | 22 |
| [web-server/src/config/mod.rs](/web-server/src/config/mod.rs) | Rust | 3 | 0 | 1 | 4 |
| [web-server/src/dbaccess/article.rs](/web-server/src/dbaccess/article.rs) | Rust | 246 | 0 | 12 | 258 |
| [web-server/src/dbaccess/login.rs](/web-server/src/dbaccess/login.rs) | Rust | 211 | 0 | 13 | 224 |
| [web-server/src/dbaccess/mod.rs](/web-server/src/dbaccess/mod.rs) | Rust | 18 | 0 | 4 | 22 |
| [web-server/src/dbaccess/perm.rs](/web-server/src/dbaccess/perm.rs) | Rust | 145 | 0 | 7 | 152 |
| [web-server/src/dbaccess/quick_msg.rs](/web-server/src/dbaccess/quick_msg.rs) | Rust | 261 | 0 | 9 | 270 |
| [web-server/src/dbaccess/role.rs](/web-server/src/dbaccess/role.rs) | Rust | 226 | 0 | 12 | 238 |
| [web-server/src/dbaccess/user.rs](/web-server/src/dbaccess/user.rs) | Rust | 294 | 0 | 17 | 311 |
| [web-server/src/handlers/article.rs](/web-server/src/handlers/article.rs) | Rust | 267 | 17 | 11 | 295 |
| [web-server/src/handlers/login.rs](/web-server/src/handlers/login.rs) | Rust | 512 | 0 | 17 | 529 |
| [web-server/src/handlers/mod.rs](/web-server/src/handlers/mod.rs) | Rust | 8 | 0 | 1 | 9 |
| [web-server/src/handlers/perm.rs](/web-server/src/handlers/perm.rs) | Rust | 190 | 22 | 8 | 220 |
| [web-server/src/handlers/quick_msg.rs](/web-server/src/handlers/quick_msg.rs) | Rust | 222 | 18 | 7 | 247 |
| [web-server/src/handlers/role.rs](/web-server/src/handlers/role.rs) | Rust | 188 | 22 | 8 | 218 |
| [web-server/src/handlers/test.rs](/web-server/src/handlers/test.rs) | Rust | 20 | 0 | 2 | 22 |
| [web-server/src/handlers/upload.rs](/web-server/src/handlers/upload.rs) | Rust | 60 | 0 | 3 | 63 |
| [web-server/src/handlers/user.rs](/web-server/src/handlers/user.rs) | Rust | 230 | 27 | 7 | 264 |
| [web-server/src/main.rs](/web-server/src/main.rs) | Rust | 65 | 0 | 12 | 77 |
| [web-server/src/middleware/auth.rs](/web-server/src/middleware/auth.rs) | Rust | 0 | 0 | 2 | 2 |
| [web-server/src/middleware/cors.rs](/web-server/src/middleware/cors.rs) | Rust | 7 | 0 | 2 | 9 |
| [web-server/src/middleware/errors.rs](/web-server/src/middleware/errors.rs) | Rust | 39 | 0 | 7 | 46 |
| [web-server/src/middleware/filter.rs](/web-server/src/middleware/filter.rs) | Rust | 66 | 0 | 6 | 72 |
| [web-server/src/middleware/mod.rs](/web-server/src/middleware/mod.rs) | Rust | 4 | 0 | 1 | 5 |
| [web-server/src/models/dto/article.rs](/web-server/src/models/dto/article.rs) | Rust | 131 | 0 | 31 | 162 |
| [web-server/src/models/dto/base.rs](/web-server/src/models/dto/base.rs) | Rust | 30 | 0 | 9 | 39 |
| [web-server/src/models/dto/login.rs](/web-server/src/models/dto/login.rs) | Rust | 168 | 0 | 41 | 209 |
| [web-server/src/models/dto/mod.rs](/web-server/src/models/dto/mod.rs) | Rust | 7 | 0 | 1 | 8 |
| [web-server/src/models/dto/perm.rs](/web-server/src/models/dto/perm.rs) | Rust | 100 | 0 | 35 | 135 |
| [web-server/src/models/dto/quick_msg.rs](/web-server/src/models/dto/quick_msg.rs) | Rust | 83 | 0 | 27 | 110 |
| [web-server/src/models/dto/role.rs](/web-server/src/models/dto/role.rs) | Rust | 66 | 0 | 20 | 86 |
| [web-server/src/models/dto/user.rs](/web-server/src/models/dto/user.rs) | Rust | 96 | 0 | 30 | 126 |
| [web-server/src/models/entity/base.rs](/web-server/src/models/entity/base.rs) | Rust | 15 | 0 | 2 | 17 |
| [web-server/src/models/entity/mod.rs](/web-server/src/models/entity/mod.rs) | Rust | 3 | 0 | 1 | 4 |
| [web-server/src/models/entity/user.rs](/web-server/src/models/entity/user.rs) | Rust | 13 | 0 | 3 | 16 |
| [web-server/src/models/entity/user_detail.rs](/web-server/src/models/entity/user_detail.rs) | Rust | 20 | 0 | 3 | 23 |
| [web-server/src/models/enums/article.rs](/web-server/src/models/enums/article.rs) | Rust | 99 | 0 | 14 | 113 |
| [web-server/src/models/enums/common.rs](/web-server/src/models/enums/common.rs) | Rust | 46 | 0 | 6 | 52 |
| [web-server/src/models/enums/mod.rs](/web-server/src/models/enums/mod.rs) | Rust | 2 | 0 | 0 | 2 |
| [web-server/src/models/mod.rs](/web-server/src/models/mod.rs) | Rust | 4 | 0 | 1 | 5 |
| [web-server/src/models/vo/article.rs](/web-server/src/models/vo/article.rs) | Rust | 43 | 0 | 3 | 46 |
| [web-server/src/models/vo/login.rs](/web-server/src/models/vo/login.rs) | Rust | 62 | 0 | 6 | 68 |
| [web-server/src/models/vo/mod.rs](/web-server/src/models/vo/mod.rs) | Rust | 7 | 0 | 1 | 8 |
| [web-server/src/models/vo/perm.rs](/web-server/src/models/vo/perm.rs) | Rust | 37 | 0 | 4 | 41 |
| [web-server/src/models/vo/quick_msg.rs](/web-server/src/models/vo/quick_msg.rs) | Rust | 30 | 0 | 2 | 32 |
| [web-server/src/models/vo/role.rs](/web-server/src/models/vo/role.rs) | Rust | 20 | 0 | 3 | 23 |
| [web-server/src/models/vo/upload.rs](/web-server/src/models/vo/upload.rs) | Rust | 8 | 0 | 2 | 10 |
| [web-server/src/models/vo/user.rs](/web-server/src/models/vo/user.rs) | Rust | 32 | 0 | 3 | 35 |
| [web-server/src/routers/article.rs](/web-server/src/routers/article.rs) | Rust | 14 | 0 | 1 | 15 |
| [web-server/src/routers/assets.rs](/web-server/src/routers/assets.rs) | Rust | 7 | 0 | 1 | 8 |
| [web-server/src/routers/common.rs](/web-server/src/routers/common.rs) | Rust | 12 | 0 | 1 | 13 |
| [web-server/src/routers/login.rs](/web-server/src/routers/login.rs) | Rust | 17 | 0 | 1 | 18 |
| [web-server/src/routers/mod.rs](/web-server/src/routers/mod.rs) | Rust | 64 | 0 | 3 | 67 |
| [web-server/src/routers/perm.rs](/web-server/src/routers/perm.rs) | Rust | 14 | 0 | 1 | 15 |
| [web-server/src/routers/quick_msg.rs](/web-server/src/routers/quick_msg.rs) | Rust | 14 | 0 | 2 | 16 |
| [web-server/src/routers/role.rs](/web-server/src/routers/role.rs) | Rust | 15 | 0 | 1 | 16 |
| [web-server/src/routers/test.rs](/web-server/src/routers/test.rs) | Rust | 5 | 0 | 1 | 6 |
| [web-server/src/routers/upload.rs](/web-server/src/routers/upload.rs) | Rust | 8 | 0 | 2 | 10 |
| [web-server/src/routers/user.rs](/web-server/src/routers/user.rs) | Rust | 14 | 0 | 1 | 15 |
| [web-server/src/utils/captcha.rs](/web-server/src/utils/captcha.rs) | Rust | 25 | 1 | 4 | 30 |
| [web-server/src/utils/db.rs](/web-server/src/utils/db.rs) | Rust | 12 | 0 | 2 | 14 |
| [web-server/src/utils/email.rs](/web-server/src/utils/email.rs) | Rust | 36 | 3 | 6 | 45 |
| [web-server/src/utils/epc.rs](/web-server/src/utils/epc.rs) | Rust | 14 | 3 | 4 | 21 |
| [web-server/src/utils/fields.rs](/web-server/src/utils/fields.rs) | Rust | 82 | 15 | 5 | 102 |
| [web-server/src/utils/ip.rs](/web-server/src/utils/ip.rs) | Rust | 42 | 0 | 5 | 47 |
| [web-server/src/utils/jwt.rs](/web-server/src/utils/jwt.rs) | Rust | 126 | 20 | 14 | 160 |
| [web-server/src/utils/log.rs](/web-server/src/utils/log.rs) | Rust | 45 | 7 | 6 | 58 |
| [web-server/src/utils/mod.rs](/web-server/src/utils/mod.rs) | Rust | 14 | 0 | 1 | 15 |
| [web-server/src/utils/permission.rs](/web-server/src/utils/permission.rs) | Rust | 13 | 0 | 2 | 15 |
| [web-server/src/utils/qrcode.rs](/web-server/src/utils/qrcode.rs) | Rust | 9 | 0 | 1 | 10 |
| [web-server/src/utils/redis.rs](/web-server/src/utils/redis.rs) | Rust | 48 | 8 | 8 | 64 |
| [web-server/src/utils/time.rs](/web-server/src/utils/time.rs) | Rust | 6 | 1 | 2 | 9 |
| [web-server/src/utils/validate.rs](/web-server/src/utils/validate.rs) | Rust | 38 | 1 | 3 | 42 |
| [web-server/src/utils/wxapp.rs](/web-server/src/utils/wxapp.rs) | Rust | 302 | 25 | 27 | 354 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)