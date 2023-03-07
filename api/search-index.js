var searchIndex = JSON.parse('{\
"cansat_core":{"doc":"Platform agnostic library for cansat.","t":[5,0,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["calculate_altitude","quantity","Pressure","add","as_hectos","as_pascals","borrow","borrow_mut","clone","div","div","eq","from","from_hectos","from_pascals","into","mul","partial_cmp","sub","try_from","try_into","type_id"],"q":["cansat_core","","cansat_core::quantity","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","",""],"i":[0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[[1,2],0,0,[[1,1],1],[1,2],[1,2],[[]],[[]],[1,1],[1,1],[[1,1]],[[1,1],3],[[]],[2,1],[2,1],[[]],[1,1],[[1,1],[[5,[4]]]],[[1,1],1],[[],6],[[],6],[[],7]],"p":[[3,"Pressure"],[15,"f32"],[15,"bool"],[4,"Ordering"],[4,"Option"],[4,"Result"],[3,"TypeId"]]},\
"cansat_gps":{"doc":"Gps device driver using embedded-hal traits.","t":[4,3,17,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Error","Gps","MAX_NMEA_LEN","Overflow","Serial","borrow","borrow","borrow_mut","borrow_mut","fmt","from","from","into","into","last_nmea","new","read_serial","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["cansat_gps","","","","","","","","","","","","","","","","","","","","","",""],"d":["","Gps driver.","Maximum length of an NMEA message including $ and [CR][LF].","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Reads last received NMEA message.","","Reads a single character from serial in a blocking mode …","","","","","",""],"i":[0,0,0,1,1,4,1,4,1,1,4,1,4,1,4,4,4,4,1,4,1,4,1],"f":[0,0,0,0,0,[[]],[[]],[[]],[[]],[[1,2],3],[[]],[[]],[[]],[[]],[4,[[7,[[6,[5]]]]]],[[],4],[4,[[8,[1]]]],[[],8],[[],8],[[],8],[[],8],[[],9],[[],9]],"p":[[4,"Error"],[3,"Formatter"],[6,"Result"],[3,"Gps"],[15,"u8"],[3,"Vec"],[4,"Option"],[4,"Result"],[3,"TypeId"]]},\
"cansat_stm32f4":{"doc":"Binary crate targeting stm32f4 family microcontrollers.","t":[6,6,6,6,3,6,6,6,17,17,6,6,6,6,6,6,3,6,6,6,6,6,6,0,11,11,12,0,12,11,11,11,0,0,0,11,11,11,12,5,3,6,3,5,5,2,3,3,3,3,5,5,5,3,3,3,3,3,3,3,0,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,0,0,5,0,5,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,0,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,0,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,2,3,0,2,12,2,2,2,2,2,2,2,3,12,2,3,3,12,12,12,12,2,3,2,12,0,2,5,13,4,3,13,13,8,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,6,17,3,11,11,12,11,12,11,11,11,11,11,11,12,11,3,3,12,11,11,11,11,12,12,12,11,11,12,12,5,5,11,11,12,12,12,12,12,12,12,11,11,11,11,11,11,5,5,5],"n":["BlockSpi2","Bme280","Cs2","Delay","DummyClock","Gps","I2c1","Led","MAX_OPEN_DIRS","MAX_OPEN_FILES","Miso2","Monotonic","Mosi2","Rx1","Sck2","Scl1","SdLogger","Sda1","SdmmcController","Serial1","Spi2","Spi2Device","Tx1","app","borrow","borrow_mut","controller","error","file","from","get_timestamp","into","sd_logger","startup","tasks","try_from","try_into","type_id","volume","EXTI0","Local","MicrosecMono","Shared","TIM2","USART1","_","__rtic_internal_Monotonics","__rtic_internal_blinkLocalResources","__rtic_internal_blink_Context","__rtic_internal_blink_MicrosecMono_SpawnHandle","__rtic_internal_blink_MicrosecMono_spawn_after","__rtic_internal_blink_MicrosecMono_spawn_at","__rtic_internal_blink_spawn","__rtic_internal_gps_irqSharedResources","__rtic_internal_gps_irq_Context","__rtic_internal_idleLocalResources","__rtic_internal_idleSharedResources","__rtic_internal_idle_Context","__rtic_internal_initLocalResources","__rtic_internal_init_Context","blink","bme280","bme280","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cancel","core","cs","delay","delay","device","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","gps","gps","gps","gps_irq","idle","idle","init","init","into","into","into","into","into","into","into","into","into","into","into","into","into","led","led","local","local","local","monotonics","new","new","new","new","new","new","new","new","new","reschedule_after","reschedule_at","sd_logger","sd_logger","shared","shared","shared_resources","spi2_device","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","Context","LocalResources","MicrosecMono","SpawnHandle","led","spawn","spawn_after","spawn_at","SpawnHandle","spawn_after","spawn_at","Context","SharedResources","gps","Context","LocalResources","SharedResources","bme280","delay","gps","sd_logger","Context","LocalResources","Monotonics","spi2_device","MicrosecMono","now","now","Bme280","Error","Report","Sdmmc","SerialConfig","WrapErr","borrow","borrow","borrow_mut","borrow_mut","cause","description","fmt","fmt","format","format","from","from","from","from","from","from","from","into","into","try_from","try_from","try_into","try_into","type_id","type_id","wrap_err","Error","FILENAME","SdLogger","borrow","borrow_mut","controller","drop","file","from","into","new","try_from","try_into","type_id","volume","write","Board","CanSat","bme280","borrow","borrow","borrow_mut","borrow_mut","cs2","delay","delay","from","from","gps","i2c1","init_board","init_drivers","into","into","led","led","monotonic","monotonic","sd_logger","serial1","spi2","try_from","try_from","try_into","try_into","type_id","type_id","blink","gps_irq","idle"],"q":["cansat_stm32f4","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cansat_stm32f4::app","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cansat_stm32f4::app::blink","","","","","","","","cansat_stm32f4::app::blink::MicrosecMono","","","cansat_stm32f4::app::gps_irq","","","cansat_stm32f4::app::idle","","","","","","","cansat_stm32f4::app::init","","","","cansat_stm32f4::app::monotonics","","cansat_stm32f4::app::monotonics::MicrosecMono","cansat_stm32f4::error","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cansat_stm32f4::sd_logger","","","","","","","","","","","","","","","","cansat_stm32f4::startup","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cansat_stm32f4::tasks","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","The RTIC application module","","","","Error reporting utilities such as <code>Report</code>, <code>Error</code> and <code>WrapErr</code>…","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","Interrupt handler to dispatch tasks at priority 1","","User code from within the module","","","","","Monotonics used by the system","Local resources <code>blink</code> has access to","Execution context","","Spawns the task after a set duration relative to the …","Spawns the task at a fixed time instant","Spawns the task directly","Shared resources <code>gps_irq</code> has access to","Execution context","Local resources <code>idle</code> has access to","Shared resources <code>idle</code> has access to","Execution context","Local resources <code>init</code> has access to","Execution context","Software task","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Core (Cortex-M) peripherals","Critical section token for init","","","Device peripherals","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Hardware task","Idle loop","","Initialization function","User code end","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Local Resources this task has access to","Local Resources this task has access to","Local Resources this task has access to","Holds static methods for each monotonic.","","","","","","","","","","","","","","Shared Resources this task has access to","Shared Resources this task has access to","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Local resources <code>blink</code> has access to","","","","","","","","","","","Shared resources <code>gps_irq</code> has access to","","","Local resources <code>idle</code> has access to","Shared resources <code>idle</code> has access to","","","","","","Local resources <code>init</code> has access to","","","This module holds the static implementation for …","","Read the current time from this monotonic","","Aggregate for all the possible errors.","Error report consisting of an description and an <code>Error</code>.","","","Extension trait for <code>Result</code> that maps the error to <code>Report</code>.","","","","","","","","","","","","Returns the argument unchanged.","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","Toggles led every second","USART3 interrupt handler that reads data into the gps …",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,30,0,30,1,1,1,0,0,0,1,1,1,30,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,42,15,43,42,11,44,10,14,15,8,16,17,18,19,5,43,42,11,44,10,14,15,8,16,17,18,19,5,5,10,10,42,15,10,5,43,42,11,44,10,14,15,8,16,17,18,19,5,43,14,16,0,0,0,0,0,43,42,11,44,10,14,15,8,16,17,18,19,5,42,18,10,8,19,0,11,10,14,15,8,16,17,18,19,5,5,42,15,8,17,0,11,43,42,11,44,10,14,15,8,16,17,18,19,5,43,42,11,44,10,14,15,8,16,17,18,19,5,43,42,11,44,10,14,15,8,16,17,18,19,5,0,0,0,0,18,0,0,0,0,0,0,0,0,16,0,0,0,15,15,14,15,0,0,0,11,0,0,0,21,0,0,21,21,0,20,21,20,21,20,20,20,21,20,21,20,20,21,21,21,21,21,20,21,20,21,20,21,20,21,45,0,0,0,30,30,30,30,30,30,30,30,30,30,30,30,30,0,0,41,41,39,41,39,39,41,39,41,39,41,39,0,0,41,39,41,39,41,39,41,39,39,41,39,41,39,41,39,0,0,0],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],0,0,0,[[]],[1,2],[[]],0,0,0,[[],3],[[],3],[[],4],0,[[]],0,0,0,[[]],[[]],0,0,0,0,0,[[],[[3,[5]]]],[[],[[3,[5]]]],[[],3],0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[5,3],0,0,0,0,0,[[5,6],7],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,[8,9],0,[10],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,[[],11],[12,10],[13,14],[[],15],[13,8],[13,16],[13,17],[[],18],[13,19],[5,[[3,[5]]]],[5,[[3,[5]]]],0,0,0,0,0,0,[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],0,0,0,0,0,0,[[]],[[]],[[]],[[]],0,0,[[20,6],7],[[21,6],7],[[20,22]],[[21,22]],[[[23,[21]]],20],[[]],[24,21],[[[26,[25]]],21],[27,21],[[]],[[[28,[24]]],21],[[]],[[]],[[],3],[[],3],[[],3],[[],3],[[],4],[[],4],[29,[[3,[20]]]],0,0,0,[[]],[[]],0,[30],0,[[]],[[]],[[[35,[[34,[31,[33,[32]]]],1]]],[[3,[30,[28,[24]]]]]],[[],3],[[],3],[[],4],0,[30,[[3,[36,37]]]],0,0,0,[[]],[[]],[[]],[[]],0,0,0,[[]],[[]],0,0,[38,39],[[39,40],[[3,[41,20]]]],[[]],[[]],0,0,0,0,0,0,0,[[],3],[[],3],[[],3],[[],3],[[],4],[[],4],[19],[17],[8,9]],"p":[[3,"DummyClock"],[3,"Timestamp"],[4,"Result"],[3,"TypeId"],[3,"__rtic_internal_blink_MicrosecMono_SpawnHandle"],[3,"Formatter"],[6,"Result"],[3,"__rtic_internal_idle_Context"],[15,"never"],[3,"__rtic_internal_init_Context"],[3,"__rtic_internal_initLocalResources"],[3,"Peripherals"],[3,"Priority"],[3,"__rtic_internal_idleSharedResources"],[3,"__rtic_internal_idleLocalResources"],[3,"__rtic_internal_gps_irqSharedResources"],[3,"__rtic_internal_gps_irq_Context"],[3,"__rtic_internal_blinkLocalResources"],[3,"__rtic_internal_blink_Context"],[3,"Report"],[4,"Error"],[3,"Formatter"],[8,"Into"],[4,"Error"],[4,"Error"],[4,"Error"],[3,"InvalidConfig"],[4,"Error"],[15,"str"],[3,"SdLogger"],[6,"Spi2"],[3,"Output"],[6,"PB12"],[3,"BlockSpi"],[3,"Controller"],[15,"usize"],[8,"Debug"],[3,"Peripherals"],[3,"Board"],[4,"Option"],[3,"CanSat"],[3,"Local"],[3,"Shared"],[3,"__rtic_internal_Monotonics"],[8,"WrapErr"]]},\
"cansat_test_utils":{"doc":"Test utilities.","t":[0,3,11,11,11,11,11,11,11,11,11],"n":["mock","Serial","borrow","borrow_mut","from","into","new","read","try_from","try_into","type_id"],"q":["cansat_test_utils","cansat_test_utils::mock","","","","","","","","",""],"d":["Mock types.","Mock type implementing <code>embedded_hal::serial</code> traits.","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Creates an instance of serial mock that will output <code>data</code> …","Reads a single byte from the serial.","","",""],"i":[0,0,2,2,2,2,2,2,2,2,2],"f":[0,0,[[]],[[]],[[]],[[]],[1,2],[[[2,[3]]],[[5,[4]]]],[[],6],[[],6],[[],7]],"p":[[8,"IntoIterator"],[3,"Serial"],[8,"Iterator"],[15,"u8"],[6,"Result"],[4,"Result"],[3,"TypeId"]]},\
"xtask":{"doc":"Custom commands for cargo.","t":[13,3,4,13,13,11,11,11,11,11,11,11,11,12,11,11,5,11,11,11,11,11,11,11,11,11,11,5,5,11,11,11,11,11,11,11,11,11,11,5,12,12,12,12],"n":["Build","Cli","Cmd","Embed","Test","augment_args","augment_args_for_update","augment_subcommands","augment_subcommands_for_update","borrow","borrow","borrow_mut","borrow_mut","cmd","command","command_for_update","format_cmd_args","from","from","from_arg_matches","from_arg_matches","from_arg_matches_mut","from_arg_matches_mut","group_id","has_subcommand","into","into","main","run","try_from","try_from","try_into","try_into","type_id","type_id","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches_mut","update_from_arg_matches_mut","workspace_members","args","args","args","pkg_name"],"q":["xtask","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","xtask::Cmd","","",""],"d":["<code>cd</code> and build each package","","","Run <code>cargo embed</code> on a specified package","<code>cd</code> and test each package","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","Arguments for <code>cargo embed</code>","Arguments for <code>cargo build</code>","Arguments for <code>cargo test</code>","Package name"],"i":[8,0,0,8,8,5,5,8,8,5,8,5,8,5,5,5,0,5,8,5,8,5,8,5,8,5,8,0,0,5,8,5,8,5,8,5,8,5,8,0,16,17,18,16],"f":[0,0,0,0,0,[1,1],[1,1],[1,1],[1,1],[[]],[[]],[[]],[[]],0,[[],1],[[],1],[[],[[3,[2]]]],[[]],[[]],[4,[[7,[5,6]]]],[4,[[7,[8,6]]]],[4,[[7,[5,6]]]],[4,[[7,[8,6]]]],[[],[[3,[9]]]],[10,11],[[]],[[]],[[],12],[[],13],[[],7],[[],7],[[],7],[[],7],[[],14],[[],14],[[5,4],[[7,[6]]]],[[8,4],[[7,[6]]]],[[5,4],[[7,[6]]]],[[8,4],[[7,[6]]]],[[],[[13,[[15,[2]]]]]],0,0,0,0],"p":[[3,"Command"],[3,"String"],[4,"Option"],[3,"ArgMatches"],[3,"Cli"],[6,"Error"],[4,"Result"],[4,"Cmd"],[3,"Id"],[15,"str"],[15,"bool"],[3,"ExitCode"],[6,"Result"],[3,"TypeId"],[3,"Vec"],[13,"Embed"],[13,"Build"],[13,"Test"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
