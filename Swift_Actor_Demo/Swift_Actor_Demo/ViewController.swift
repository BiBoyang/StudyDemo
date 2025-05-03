//
//  ViewController.swift
//  Swift_Actor_Demo
//
//  Created by Boyang on 2025/5/3.
//

import UIKit

// 使用 @MainActor 修饰的类，保证其所有方法和属性都在主线程执行，适合做 UI 相关操作
@MainActor class MainActorDemo {
    // 只读属性，初始值为 "Hello, MainActor!"
    private(set) var text: String = "Hello, MainActor!"

    // 更新 text 属性的方法，并打印当前线程信息
    func updateText(newText: String) {
        text = newText
        print("更新后的文本：\(text) (当前线程：\(Thread.isMainThread ? "主线程" : "非主线程"))")
        print("当前线程详细信息\(Thread.current)")
    }
}


class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        
        self.view.backgroundColor = UIColor.red
        
        // Actor 使用示例
        let count = Numberss()
        
        Task{
            await count.increment()
            await count.increment()
            let result = await count.getValue()
            print("Count value is:\(result) ")
        }
        
        // @MainActor 使用示例
        let demo = MainActorDemo()
        Task {
            // 这里无需 await，因为 updateText 不是异步方法，但会自动切换到主线程执行
            demo.updateText(newText: "你好，@MainActor！")
            
        }
        

        
        
    }
}

// 定义一个 Actor，保证其内部状态的线程安全
actor Numberss {
    private var value: Int = 0
    
    func increment() {
        value += 1
    }
    
    func getValue() -> Int {
        return value
    }
}
