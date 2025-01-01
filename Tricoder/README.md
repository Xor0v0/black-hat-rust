# Tricoder

## Usage

tricoder <target_domain>

## Use of crates

- 使用 thiserror 的 Error 宏进行 lib 中的错误处理；使用 anyhow 的 Error 类型进行 application 中的错误处理。
- 使用 trusted-dns-resolver 来对域名进行解析，其中 Resolver 是解析器，需要 config 和 opt：
  - ResolverConfig 用于配置「用哪些 DNS 服务器」以及「解析域名的基本信息」
  - ResolverOpts 配置「DNS 查询的具体行为选项」
- 
