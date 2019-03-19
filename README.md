# circleCI-practice
circleCI practice
# Tips
- .circleci/config.yml に書く.yamlはNG
- jobsは別々のコンテナで行う
- docker-compose config 変数補って表示してくれる
- docker-compose は yml ファイルからの相対パス
- dockerfile の build 時間を減らすために pull を build の前にする
  - 気をつけるのは最初の pull のときで
    - imageがない場合があるので仮のimageを作って空にする
- set -x シェルが実行したコマンドと引数を表示する
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

## gcloud-builders/kubectl
- サービスアカウントを調べてから権限を付与する