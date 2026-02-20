# rust_learn
echo "# rust_learn" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin https://github.com/Blederos/rust_learn.git
git push -u origin main

# 1. 修改代码后，添加所有更改的文件
git add .
# 2. 提交更改（替换成你的提交信息）
git commit -m "修改了xxx功能"
# 3. 推送到GitHub（已关联分支，无需写全参数）
git push

# 拉取远程最新代码：
如果其他人修改了 GitHub 仓库的代码，你可以用以下命令同步到本地：
git pull