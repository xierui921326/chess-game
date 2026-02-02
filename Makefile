.PHONY: help install dev build test test-frontend test-backend test-all clean lint format

# 默认目标：显示帮助信息
help:
	@echo "棋类游戏应用程序 - 可用命令："
	@echo ""
	@echo "  make install          - 安装所有依赖（前端 + 后端）"
	@echo "  make dev              - 启动开发服务器"
	@echo "  make build            - 构建生产版本"
	@echo "  make test             - 运行所有测试"
	@echo "  make test-frontend    - 仅运行前端测试"
	@echo "  make test-backend     - 仅运行后端测试"
	@echo "  make test-watch       - 以监视模式运行前端测试"
	@echo "  make lint             - 运行代码检查"
	@echo "  make format           - 格式化代码"
	@echo "  make clean            - 清理构建产物"
	@echo "  make git-status       - 查看 Git 状态"
	@echo "  make git-log          - 查看最近的提交记录"
	@echo ""

# 安装依赖
install:
	@echo "📦 安装前端依赖..."
	npm install
	@echo "✅ 依赖安装完成"

# 开发模式
dev:
	@echo "🚀 启动开发服务器..."
	npm run tauri dev

# 构建生产版本
build:
	@echo "🔨 构建生产版本..."
	npm run tauri build

# 运行所有测试
test: test-frontend test-backend
	@echo "✅ 所有测试完成"

# 前端测试
test-frontend:
	@echo "🧪 运行前端测试..."
	npm run test:run

# 后端测试
test-backend:
	@echo "🧪 运行后端测试..."
	cd src-tauri && cargo test

# 监视模式测试
test-watch:
	@echo "👀 以监视模式运行前端测试..."
	npm test

# 测试 UI
test-ui:
	@echo "🎨 启动测试 UI..."
	npm run test:ui

# 代码检查
lint:
	@echo "🔍 运行代码检查..."
	@echo "检查 TypeScript..."
	npm run build
	@echo "检查 Rust..."
	cd src-tauri && cargo clippy -- -D warnings

# 格式化代码
format:
	@echo "✨ 格式化代码..."
	@echo "格式化 Rust 代码..."
	cd src-tauri && cargo fmt
	@echo "✅ 代码格式化完成"

# 清理构建产物
clean:
	@echo "🧹 清理构建产物..."
	rm -rf dist
	rm -rf src-tauri/target
	rm -rf node_modules
	@echo "✅ 清理完成"

# Git 相关命令
git-status:
	@echo "📊 Git 状态："
	@git status

git-log:
	@echo "📜 最近的提交记录："
	@git log -n 10 --pretty=format:'%C(yellow)%h%Creset %C(blue)%ad%Creset %C(green)%s%Creset' --date=short

git-diff:
	@echo "📝 未提交的更改："
	@git diff

# 运行特定的测试文件
test-file:
	@echo "🧪 运行指定测试文件..."
	@if [ -z "$(FILE)" ]; then \
		echo "❌ 请指定测试文件: make test-file FILE=path/to/test.ts"; \
	else \
		npm test -- $(FILE); \
	fi

# 检查项目健康状态
health-check: test lint
	@echo "✅ 项目健康检查完成"

# 快速检查（不运行完整测试）
quick-check:
	@echo "⚡ 快速检查..."
	@echo "检查 TypeScript 编译..."
	npm run build
	@echo "检查 Rust 编译..."
	cd src-tauri && cargo check
	@echo "✅ 快速检查完成"
