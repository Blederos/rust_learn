* Rust的内存安全意味着什么
  
  * 核心原则
    
    * Rust从根源避免内存相关错误，安全Rust中无法编写内存不安全代码
    
    * 将内存安全的责任从程序员转移到编译器，编译期强制校验规则
    
    * 原始指针仅允许在unsafe不安全块中使用，实现安全/不安全代码分离
  
  * 消除空指针错误的设计
    
    * Rust无空指针概念，从根源解决空指针解引用问题
    
    * 提供Option枚举类型，表示值的存在/缺失，需显式处理该类型
    
    * 引用始终指向有效、非空的内存，编译期强制执行有效性
    
    * 所有变量使用前必须完成初始化，未初始化代码无法编译
    
    * 可通过match语句判断Option类型的值是存在还是缺失
  
  * 避免释放后使用、悬垂指针问题
    
    * 释放后使用：程序访问已被释放的内存，C/C++手动内存管理的常见错误
    
    * 悬垂指针：程序一部分释放内存，另一部分仍访问该内存，导致未定义行为
    
    * 核心解决特性：所有权系统、借用检查器、生命周期
    
    * 生命周期确保引用在使用期间有效，且不会比指向的数据存活更久
    
    * 借用检查器会检测内存借用冲突，存在问题的代码直接无法编译
    
    * C/C++中需程序员手动规避此类问题，易因人为错误引入漏洞
    
    * Rust编译器报错信息具备指导性，可清晰定位内存相关问题
  
  * 杜绝双重释放错误的机制
    
    * 双重释放：程序多次释放同一内存位置，C/C++中会导致未定义行为
    
    * 安全Rust中不直接暴露原始指针，通过智能指针处理内存操作
    
    * 内存管理由所有权模型和智能指针自动完成，无需手动调用free/delete
  
  * Rust堆内存的动态分配与自动管理
    
    * 可通过Box智能指针实现堆内存的动态分配
    
    * 智能指针处理原始指针，对用户隐藏原始指针操作
    
    * 智能指针变量离开作用域时，其指向的堆内存会被自动解除分配
    
    * 原理类似C++智能指针，通过析构函数自动释放内存
    
    * 编译器会为实现drop trait的类型，在编译期自动插入释放调用
  
  * 实用工具与编译器内部视角
    
    * 在线编译器：Rust Playground，可在线编写、运行Rust代码
    
    * MIR中级表示：可查看编译器编译期间对代码的转换、优化和分析过程
  
  * 课程相关安排
    
    * 设专门章节讲解引用、Option/Result类型、枚举、结构体、trait等核心内容
    
    * 设专门章节讲解原始指针、智能指针，会更新智能指针高级主题及用例
    
    * 会透彻讲解match语句与各类数据类型的结合使用
    
    * 会详细讲解所有权、借用检查器、移动/借用操作等核心特性
  
  * 后续讲解规划
    
    * 探讨Rust处理缓冲区溢出、避免数据竞争的策略
    
    * 下一讲从Rust基础开始，学习控制台打印消息的方法
    
    * 完成基础部分后，继续讲解Rust内存安全特性的剩余内容
    
    * 
  
```rust
  fn main(){
  let x: Option<&i32> = None;
  println!("{:?}",x);
  
      match x {
          Some(x) => {
              println!("{}",x);
          }
          None => {
              println!("None");
          }
      }
  }
```
  
  error:Cannot move out of original ptr' because it is borrowed
  
```rust
  fn main(){
  //allocates heap memory to store integer 42
  let original_ptr =Box::new(42);
  //Borrow smart pointer
  let copied_ptr = &original_ptr;
  //Attempting to drop original_ptr while copied_ptr still borrows it
  drop(original_ptr);
  
  //borrowed smart pointer later used here
  println!("value:{}",copied_ptr);
  }
```

```rust
fn store_data_on_heap(data:u32)
{
    let heap_value = Box::new(data);//分配堆上的内存
    println!("heap value is {}", heap_value);
    //Memory is automatically deallocated here when heap_value`goes out of scope
}

fn main(){
    store_data_on_heap(100);
}
```
