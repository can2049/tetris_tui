

# Tetris (Rust + Ratatui)

## 项目目标

实现一个 **基于 Rust 的终端版俄罗斯方块（Tetris）游戏**，
使用 **ratatui** 作为 TUI（Terminal UI）框架，
在 Linux / macOS 终端环境下可运行。

项目目标是：

* 结构清晰、模块职责明确
* 易于扩展（计分、关卡、暂停等）
* 不依赖图形界面，仅使用终端渲染

---

## 技术选型

* **语言**：Rust（Edition 2024）
* **TUI 框架**：ratatui
* **终端后端**：crossterm
* **随机数**：rand
* **时间控制**：std::time

---

## 功能需求

### 核心玩法

1. 标准俄罗斯方块 10 × 20 游戏区域
2. 支持 7 种 Tetromino（I, O, T, S, Z, J, L）
3. 方块自动下落
4. 行满后消除并计分
5. 方块堆叠至顶部时游戏结束

---

### 用户输入

| 按键    | 功能              |
| ----- | --------------- |
| ← / h | 向左移动            |
| → / l | 向右移动            |
| ↓ / j | 加速下落            |
| ↑ / k | 顺时针旋转           |
| Space | 直接下落（hard drop） |
| p     | 暂停 / 继续         |
| q     | 退出游戏            |


如果用户选择退出，需要给出退出确认界面，确认用户是否真的要退出。

---

### 界面布局（ratatui）

终端界面应至少包含：

```
+--------------------+  +--------------+
|                    |  |  Next Piece  |
|                    |  |              |
|     Game Board     |  +--------------+
|                    |
|                    |  Score: 000123
|                    |  Lines: 12
+--------------------+
```

* 左侧：主游戏区域（10 × 20）
* 右侧：下一个方块预览 + 分数信息

---

## 项目结构（强制）

Cursor 生成代码时 **必须遵循以下模块划分**：

```
src/
 ├── main.rs
 ├── app.rs          # 应用主循环 / 状态机
 ├── game/
 │    ├── mod.rs
 │    ├── board.rs   # 游戏棋盘与碰撞检测
 │    ├── piece.rs   # 方块定义与旋转
 │    └── state.rs   # 游戏状态（分数、行数、结束）
 ├── ui/
 │    ├── mod.rs
 │    └── render.rs  # ratatui 渲染逻辑
 └── input.rs        # 键盘事件处理
```

---

## 核心数据结构要求

### Tetromino

```rust
enum TetrominoType {
    I, O, T, S, Z, J, L
}

struct Piece {
    kind: TetrominoType,
    rotation: u8,
    x: i32,
    y: i32,
}
```

* 旋转必须基于预定义的 shape 表
* 坐标系统：左上角为 (0, 0)

---

### Board

```rust
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

struct Board {
    cells: [[Option<TetrominoType>; BOARD_WIDTH]; BOARD_HEIGHT];
}
```

必须提供：

* 碰撞检测
* 固定方块
* 消行逻辑

---

### GameState

```rust
struct GameState {
    board: Board,
    current: Piece,
    next: Piece,
    score: u32,
    lines: u32,
    game_over: bool,
    paused: bool,
}
```

---

## 主循环逻辑（必须）

1. 初始化终端（进入 raw mode）
2. 初始化游戏状态
3. 事件循环：

   * 定时器驱动自动下落
   * 非阻塞键盘输入
   * 状态更新
   * UI 重绘
4. 游戏结束后显示提示
5. 退出时恢复终端状态

---

## 渲染要求

* 使用 ratatui 的 `Block`, `Paragraph`, `Canvas` 或 `Table`
* 方块使用 **全角字符或 Unicode block** 渲染（如 `█`)
* 不允许直接使用 println 进行绘制

---

## 错误与健壮性要求

* 终端异常退出时必须恢复终端状态
* 禁止 panic 作为正常控制流
* 使用 `Result` 处理初始化错误

---

## 可选扩展（非必须）

Cursor 可以在核心功能完成后，**作为 bonus** 实现：

* 等级（level）与下落速度递增
* 保存最高分
* 彩色方块
* 幽灵方块（ghost piece）

---

## 代码风格与约束

* 不使用 `unsafe`
* 不使用全局可变变量
* 模块间通过明确接口交互
* 尽量保持函数短小、单一职责

---

## 期望结果

运行：

```bash
cargo run
```

在终端中启动一个**可交互、可玩的俄罗斯方块游戏**。

---

如果你愿意，下一步我可以：

* 帮你 **拆解为 Cursor 多轮生成提示（step-by-step prompts）**
* 或直接 **补充 Cargo.toml**
* 或设计 **Piece 旋转表（shape definitions）** 的标准实现方式
