# rCore-Tutorial-Code

## 代码

- [实验源码](https://github.com/LearningOS/rCore-Tutorial-Code)

## 文档

- 简明手册：[rCore-Tutorial-Guide](https://LearningOS.github.io/rCore-Tutorial-Guide/)

- 详细书籍 [rCore-Tutorial-Book-v3](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)

## rCore 教程代码的 OS API 文档

- [ch1 的 OS API 文档](https://learningos.github.io/rCore-Tutorial-Code/ch1/os/index.html)
  以及 [ch2 的 OS API 文档](https://learningos.github.io/rCore-Tutorial-Code/ch2/os/index.html)
- [ch3 的 OS API 文档](https://learningos.github.io/rCore-Tutorial-Code/ch3/os/index.html)
  以及 [ch4 的 OS API 文档](https://learningos.github.io/rCore-Tutorial-Code/ch4/os/index.html)
- [ch5 的 OS API 文档](https://learningos.github.io/rCore-Tutorial-Code/ch5/os/index.html)
  以及 [ch6 的 OS API 文档](https://learningos.github.io/rCore-Tutorial-Code/ch6/os/index.html)
- [ch7 的 OS API 文档](https://learningos.github.io/rCore-Tutorial-Code/ch7/os/index.html)
  以及 [ch8 的 OS API 文档](https://learningos.github.io/rCore-Tutorial-Code/ch8/os/index.html)
- [ch9 的 OS API 文档](https://learningos.github.io/rCore-Tutorial-Code/ch9/os/index.html)

## 相关资源

- [学习资源](https://github.com/LearningOS/rust-based-os-comp2025/blob/main/relatedinfo.md)

## 设置

```bash
$ git clone https://github.com/LearningOS/2025a-rcore-[YOUR_USER_NAME].git
$ cd 2025a-rcore-[YOUR_USER_NAME]
```

## 构建与运行

```bash
# 首先设置构建和运行环境
$ git clone https://github.com/LearningOS/rCore-Tutorial-Test.git user
$ git checkout ch$ID
$ cd os
# 在 ch$ID 中运行 OS
$ make run
```

如果您想使用 Docker 来构建和运行，可以使用以下命令：
```bash
# 将 `rCore-Tutorial-Test` 仓库克隆到本地后，您可以使用以下命令来构建和运行：
$ make build_docker
$ make docker
```

如果在 Docker 中访问国外资源（如 GitHub）时遇到网络问题，您可以根据您的阶段按照以下建议操作：

- Docker pull：
  1. 使用代理：https://docs.docker.com/reference/cli/docker/image/pull/#proxy-configuration

  2. 使用可用的国内源（自行搜索）

- Docker build：使用代理 https://docs.docker.com/engine/cli/proxy/#build-with-a-proxy-configuration

- Docker run：使用代理选项，相关操作类似于 `Docker build`，可以自行参考相关资料

注意：$ID 范围为 [1-9]

## 评分

```bash
# 首先设置构建和运行环境
$ rm -rf ci-user
$ git clone https://github.com/LearningOS/rCore-Tutorial-Checker.git ci-user
$ git clone https://github.com/LearningOS/rCore-Tutorial-Test.git ci-user/user
$ git checkout ch$ID
# 使用更多测试检查并评分 ch$ID 中的 OS
$ cd ci-user && make test CHAPTER=$ID
```

注意：$ID 范围为 [3,4,5,6,8]
