use std::sync::{Arc, Mutex, RwLock};
use std::{io, thread};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use lazy_static::lazy_static;
use time::{OffsetDateTime};
use time::macros::offset;

type PlayerId = u32;

const GAME_SIZE: usize = 1000;

type WaitingList = Vec<PlayerId>;


// 使用 lazy_static! 宏定义全局 Mutex 控制的变量
lazy_static! {
    static ref PLAYER_COUNT: Mutex<AtomicU32> =
        Mutex::new(AtomicU32::new(0));
}

#[derive(Debug)]
struct AppConfig {
    mushroom_enabled: bool
}

impl AppConfig {
    fn load() -> io::Result<AppConfig> {
        let mut app_config = String::new();
        let conf = File::open("./config/fern.config")?;
        let mut conf_buf = BufReader::new(conf);
        conf_buf.read_line(&mut app_config)?;
        let result = app_config.split_once('=');
        match result {
            Some((_, is_enabled)) => {
                Ok(AppConfig {
                    mushroom_enabled: bool::from_str(is_enabled.trim()).unwrap()
                })
            },
            None => {
                Ok(AppConfig {
                    mushroom_enabled: false,
                })
            }
        }
    }
}

struct FernEmpireApp {
    waiting_list: Mutex<WaitingList>,
    config: RwLock<AppConfig>
}

fn set_app() -> Arc<FernEmpireApp> {
    Arc::new(FernEmpireApp {
        /*
        创建互斥锁，Mutex 只和锁有关,
        Arc 和 Rc 意味着堆的分配，
        如果需要在堆上创建 Mutex ，
        则需要显示创建,
        Arc 可以很方便的在多个线程之间共享内容，
        Mutex 可以很方便地在线程之间共享可变的数据
         */
        waiting_list: Mutex::new(vec![]),
        config: RwLock::new(AppConfig::load().unwrap()),
    })
}


impl FernEmpireApp {


    fn start_game(&self, players: Vec<PlayerId>) {
        println!("Player list:");
        for (player_index, player_id)in players.iter().enumerate() {
            println!("No {} => {} is ready.", player_index, player_id)
        }

        println!("{} players are ready, FernEmpire start!", &players.len());
    }

    /// 将玩家添加到等待列表中。
    /// 当玩家人数足够的时候启动游戏
    fn join_waiting_list(&self, player: PlayerId) {
        // 获取 mutex 锁，访问其中的数据，
        // guard 的作用域是关键点
        // self.waiting_list.lock() 阻塞直到取得互斥锁
        // 这个方法返回的 MutexGuard<WaitingList> 是一个 &mut WaitingList 的简单包装器
        // 通过强制解引用，可直接调用 WaitingList 的方法
        let mut guard = self.waiting_list.lock().unwrap();
        let player_count = PLAYER_COUNT.lock().unwrap();
        // guard 可以借用底层数据的直接引用。
        // Rust 的生命周期保证这些引用的生命周期不会超过 guard 的生命周期
        // 在不获取锁的前提下，没有任何方式可以访问 Mutex 中的数据
        // 当 guard drop 掉之后，锁会被释放，这一般发生在代码块的结尾，不过也可以手动 drop 掉它
        // 这个方法传递的是 &self 作为参数，
        // 但是 guard.push(player) 需要 &mut self
        // 在 Rust 中， &mut 意味着独占性访问，& 意味着共享访问，
        // 但是 Mutex 提供了一种方式：锁，
        // 事实上，为了能在很多线程都有 Mutex 自身的共享访问的情况下，
        // 仍然能够提供对于内部数据的独占访问， Mutex 做的事情比 &mut 更多一点
        // 它动态地强制进行独占访问，而这一点通常在 Rust 编译器编译期间静态完成。
        // RefCell 也有同样的机制，只不过它不支持多线程，它们都是内不可变性的体现
        guard.push(player);
        player_count.fetch_add(1, Ordering::SeqCst);

        let local = OffsetDateTime::now_utc().to_offset(offset!(+8));
        println!("{} Player => {} from => {:?} current waiting player count => {}",
                    local, &player, thread::current().id(), player_count.load(Ordering::SeqCst));

        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            self.start_game(players);
            player_count.fetch_update(Ordering::SeqCst, Ordering::SeqCst,
                                        |_| Some(0)).unwrap();
        }
    }

    /// 读取某一项配置内容
    /// 读取 APPConfig 中的配置信息，
    /// 使用 RwLock::read() 方法为数据读操作添加共享锁
    fn mush_room_enabled(&self) -> bool {
        let config_guard = self.config.read().unwrap();
        config_guard.mushroom_enabled
    }

    /// 重载配置：
    /// 将配置写入 APPConfig ,
    /// 使用 RwLock::write() 方法为数据的写操作添加独占锁
    fn reload_config(&self) -> io::Result<()> {
        let new_config = AppConfig::load()?;
        let mut config_guard = self.config.write().unwrap();
        println!("current config => {:?}", config_guard);
        *config_guard = new_config;
        println!("config changed => {:?}", config_guard);
        Ok(())
    }

}

#[test]
fn test_game() {
    use rayon::prelude::*;

    let app = set_app();

    let players: Vec<PlayerId> = (0..1200).collect();
    players
        .par_iter()
        .for_each(|player| app.join_waiting_list(*player));

    println!("mushroom enabled => {}", app.mush_room_enabled());

    app.reload_config().unwrap()
}