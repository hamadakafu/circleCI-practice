# circleCI-practice
circleCI practice
# Tips
- .circleci/config.yml に書く.yamlはNG
- jobsは別々のコンテナで行う
- docker-compose config 変数補って表示してくれる
- docker-compose は yml ファイルからの相対パス
## Dockerfile
ARG hoge  
docker build . --build-arg hoge=[term]

## config.yml
run のインデントに注意
```yaml
- run:
    name: hoge
    command: echo "fuga"
```