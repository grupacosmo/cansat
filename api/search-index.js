var searchIndex = JSON.parse('{\
"cansat_core":{"doc":"Platform agnostic library for cansat.","t":[5,0,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["calculate_altitude","unit","Pressure","add","as_hectos","as_pascals","borrow","borrow_mut","clone","div","div","eq","from","from_hectos","from_pascals","into","mul","partial_cmp","sub","try_from","try_into","type_id"],"q":["cansat_core","","cansat_core::unit","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","",""],"i":[0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[[1,2],0,0,[[1,1],1],[1,2],[1,2],[[]],[[]],[1,1],[1,1],[[1,1]],[[1,1],3],[[]],[2,1],[2,1],[[]],[1,1],[[1,1],[[5,[4]]]],[[1,1],1],[[],6],[[],6],[[],7]],"p":[[3,"Pressure"],[15,"f32"],[15,"bool"],[4,"Ordering"],[4,"Option"],[4,"Result"],[3,"TypeId"]]},\
"cansat_gps":{"doc":"Gps device driver using embedded-hal traits.","t":[4,3,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Error","Gps","Overflow","Uart","borrow","borrow","borrow_mut","borrow_mut","fmt","from","from","into","into","last_nmea","new","read_uart","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["cansat_gps","","","","","","","","","","","","","","","","","","","","",""],"d":["","Gps driver.","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Reads last received NMEA message.","","Reads a single character from UART in a blocking mode and …","","","","","",""],"i":[0,0,1,1,4,1,4,1,1,4,1,4,1,4,4,4,4,1,4,1,4,1],"f":[0,0,0,0,[[]],[[]],[[]],[[]],[[1,2],3],[[]],[[]],[[]],[[]],[4,[[7,[[6,[5]]]]]],[[],4],[4,[[8,[1]]]],[[],8],[[],8],[[],8],[[],8],[[],9],[[],9]],"p":[[4,"Error"],[3,"Formatter"],[6,"Result"],[3,"Gps"],[15,"u8"],[3,"Vec"],[4,"Option"],[4,"Result"],[3,"TypeId"]]},\
"cansat_stm32f4":{"doc":"Binary crate targeting stm32f4 family microcontrollers.","t":[6,6,6,6,3,6,6,6,17,17,17,6,6,6,6,6,6,6,6,6,6,6,6,0,11,11,0,11,11,11,0,0,11,11,11,5,5,3,6,3,5,5,2,3,3,3,3,5,5,5,3,3,3,5,5,5,3,3,3,3,3,3,3,5,5,5,3,3,3,5,5,5,0,12,12,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,0,0,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,0,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,12,12,0,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,2,3,0,2,12,2,2,2,2,2,2,2,3,0,2,12,12,2,2,2,2,2,2,2,3,12,2,3,2,12,2,0,3,2,12,2,2,2,2,2,2,0,2,5,2,3,0,2,12,12,2,2,2,2,2,2,13,4,3,13,13,8,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,3,3,12,11,11,11,11,12,12,12,12,12,11,11,12,12,5,5,11,11,12,12,12,12,12,12,11,11,11,11,11,11,5,5,5,5,5],"n":["BlockSpi2","Bme280","Cs2","Delay","DummyClock","Gps","I2c1","Led","MAX_FILENAME_LEN","MAX_OPEN_DIRS","MAX_OPEN_FILES","Miso2","Monotonic","Mosi2","Rx1","Sck2","Scl1","Sda1","SdmmcController","Serial1","Spi2","Spi2Device","Tx1","app","borrow","borrow_mut","error","from","get_timestamp","into","startup","tasks","try_from","try_into","type_id","EXTI0","EXTI1","Local","MicrosecMono","Shared","TIM2","USART1","_","__rtic_internal_Monotonics","__rtic_internal_blinkLocalResources","__rtic_internal_blink_Context","__rtic_internal_blink_MicrosecMono_SpawnHandle","__rtic_internal_blink_MicrosecMono_spawn_after","__rtic_internal_blink_MicrosecMono_spawn_at","__rtic_internal_blink_spawn","__rtic_internal_bme_measureLocalResources","__rtic_internal_bme_measure_Context","__rtic_internal_bme_measure_MicrosecMono_SpawnHandle","__rtic_internal_bme_measure_MicrosecMono_spawn_after","__rtic_internal_bme_measure_MicrosecMono_spawn_at","__rtic_internal_bme_measure_spawn","__rtic_internal_gps_irqSharedResources","__rtic_internal_gps_irq_Context","__rtic_internal_initLocalResources","__rtic_internal_init_Context","__rtic_internal_log_nmeaSharedResources","__rtic_internal_log_nmea_Context","__rtic_internal_log_nmea_MicrosecMono_SpawnHandle","__rtic_internal_log_nmea_MicrosecMono_spawn_after","__rtic_internal_log_nmea_MicrosecMono_spawn_at","__rtic_internal_log_nmea_spawn","__rtic_internal_sdmmc_logLocalResources","__rtic_internal_sdmmc_log_Context","__rtic_internal_sdmmc_log_MicrosecMono_SpawnHandle","__rtic_internal_sdmmc_log_MicrosecMono_spawn_after","__rtic_internal_sdmmc_log_MicrosecMono_spawn_at","__rtic_internal_sdmmc_log_spawn","blink","bme280","bme280","bme_measure","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cancel","cancel","cancel","cancel","controller","controller","core","cs","delay","delay","device","filename","filename","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","gps","gps","gps","gps_irq","init","init","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","led","led","local","local","local","local","log_nmea","monotonics","new","new","new","new","new","new","new","new","new","new","new","new","reschedule_after","reschedule_after","reschedule_after","reschedule_after","reschedule_at","reschedule_at","reschedule_at","reschedule_at","sdmmc_log","shared","shared","shared_resources","spi2_device","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","Context","LocalResources","MicrosecMono","SpawnHandle","led","spawn","spawn_after","spawn_at","SpawnHandle","spawn_after","spawn_at","Context","LocalResources","MicrosecMono","SpawnHandle","bme280","delay","spawn","spawn_after","spawn_at","SpawnHandle","spawn_after","spawn_at","Context","SharedResources","gps","Context","LocalResources","Monotonics","spi2_device","Context","MicrosecMono","SharedResources","SpawnHandle","gps","spawn","spawn_after","spawn_at","SpawnHandle","spawn_after","spawn_at","MicrosecMono","now","now","Context","LocalResources","MicrosecMono","SpawnHandle","controller","filename","spawn","spawn_after","spawn_at","SpawnHandle","spawn_after","spawn_at","Bme280","Error","Report","Sdmmc","SerialConfig","WrapErr","borrow","borrow","borrow_mut","borrow_mut","cause","description","fmt","fmt","format","format","from","from","from","from","from","from","from","into","into","try_from","try_from","try_into","try_into","type_id","type_id","wrap_err","Board","CanSat","bme280","borrow","borrow","borrow_mut","borrow_mut","controller","cs2","delay","delay","filename","from","from","gps","i2c1","init_board","init_drivers","into","into","led","led","monotonic","monotonic","serial1","spi2","try_from","try_from","try_into","try_into","type_id","type_id","blink","bme_measure","gps_irq","log_nmea","sdmmc_log"],"q":["cansat_stm32f4","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cansat_stm32f4::app","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cansat_stm32f4::app::blink","","","","","","","","cansat_stm32f4::app::blink::MicrosecMono","","","cansat_stm32f4::app::bme_measure","","","","","","","","","cansat_stm32f4::app::bme_measure::MicrosecMono","","","cansat_stm32f4::app::gps_irq","","","cansat_stm32f4::app::init","","","","cansat_stm32f4::app::log_nmea","","","","","","","","cansat_stm32f4::app::log_nmea::MicrosecMono","","","cansat_stm32f4::app::monotonics","","cansat_stm32f4::app::monotonics::MicrosecMono","cansat_stm32f4::app::sdmmc_log","","","","","","","","","cansat_stm32f4::app::sdmmc_log::MicrosecMono","","","cansat_stm32f4::error","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cansat_stm32f4::startup","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cansat_stm32f4::tasks","","","",""],"d":["","","","","","","","","Maximal length supported by embedded_sdmmc","","","","","","","","","","","","","","","The RTIC application module","","","Error reporting utilities such as <code>Report</code>, <code>Error</code> and <code>WrapErr</code>…","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","Interrupt handler to dispatch tasks at priority 2","Interrupt handler to dispatch tasks at priority 1","","User code from within the module","","","","","Monotonics used by the system","Local resources <code>blink</code> has access to","Execution context","","Spawns the task after a set duration relative to the …","Spawns the task at a fixed time instant","Spawns the task directly","Local resources <code>bme_measure</code> has access to","Execution context","","Spawns the task after a set duration relative to the …","Spawns the task at a fixed time instant","Spawns the task directly","Shared resources <code>gps_irq</code> has access to","Execution context","Local resources <code>init</code> has access to","Execution context","Shared resources <code>log_nmea</code> has access to","Execution context","","Spawns the task after a set duration relative to the …","Spawns the task at a fixed time instant","Spawns the task directly","Local resources <code>sdmmc_log</code> has access to","Execution context","","Spawns the task after a set duration relative to the …","Spawns the task at a fixed time instant","Spawns the task directly","Software task","","","Software task","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Core (Cortex-M) peripherals","Critical section token for init","","","Device peripherals","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Hardware task","Initialization function","User code end","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Local Resources this task has access to","Local Resources this task has access to","Local Resources this task has access to","Local Resources this task has access to","Software task","Holds static methods for each monotonic.","","","","","","","","","","","","","","","","","","","","","Software task","Shared Resources this task has access to","Shared Resources this task has access to","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Local resources <code>blink</code> has access to","","","","","","","","","","","Local resources <code>bme_measure</code> has access to","","","","","","","","","","","","Shared resources <code>gps_irq</code> has access to","","","Local resources <code>init</code> has access to","","","","","Shared resources <code>log_nmea</code> has access to","","","","","","","","","This module holds the static implementation for …","","Read the current time from this monotonic","","Local resources <code>sdmmc_log</code> has access to","","","","","","","","","","","","Aggregate for all the possible errors.","Error report consisting of an description and an <code>Error</code>.","","","Extension trait for <code>Result</code> that maps the error to <code>Report</code>.","","","","","","","","","","","Returns the argument unchanged.","","","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","Toggles led every second","","USART3 interrupt handler that reads data into the gps …","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,1,1,1,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,39,21,0,40,39,12,41,11,15,16,17,18,7,19,20,5,21,22,6,23,24,8,40,39,12,41,11,15,16,17,18,7,19,20,5,21,22,6,23,24,8,7,5,6,8,39,23,11,11,39,21,11,39,23,7,5,6,8,40,39,12,41,11,15,16,17,18,7,19,20,5,21,22,6,23,24,8,40,15,17,0,0,0,40,39,12,41,11,15,16,17,18,7,19,20,5,21,22,6,23,24,8,39,19,11,20,22,24,0,0,12,11,15,16,17,18,19,20,21,22,23,24,7,5,6,8,7,5,6,8,0,16,18,0,12,40,39,12,41,11,15,16,17,18,7,19,20,5,21,22,6,23,24,8,40,39,12,41,11,15,16,17,18,7,19,20,5,21,22,6,23,24,8,40,39,12,41,11,15,16,17,18,7,19,20,5,21,22,6,23,24,8,0,0,0,0,19,0,0,0,0,0,0,0,0,0,0,21,21,0,0,0,0,0,0,0,0,15,0,0,0,12,0,0,0,0,17,0,0,0,0,0,0,0,0,0,0,0,0,0,23,23,0,0,0,0,0,0,26,0,0,26,26,0,25,26,25,26,25,25,25,26,25,26,25,25,26,26,26,26,26,25,26,25,26,25,26,25,26,42,0,0,38,38,36,38,36,38,36,38,36,38,38,36,38,36,0,0,38,36,38,36,38,36,36,36,38,36,38,36,38,36,0,0,0,0,0],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],0,[[]],[1,2],[[]],0,0,[[],3],[[],3],[[],4],[[]],[[]],0,0,0,[[]],[[]],0,0,0,0,0,[[],[[3,[5]]]],[[],[[3,[5]]]],[[],3],0,0,0,[[],[[3,[6]]]],[[],[[3,[6]]]],[[],3],0,0,0,0,0,0,0,[[],[[3,[7]]]],[[],[[3,[7]]]],[[],3],0,0,0,[[],[[3,[8]]]],[[],[[3,[8]]]],[[],3],0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[7,3],[5,3],[6,3],[8,3],0,0,0,0,0,0,0,0,0,[[7,9],10],[[5,9],10],[[6,9],10],[[8,9],10],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,[11],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,0,[[],12],[13,11],[14,15],[14,16],[14,17],[14,18],[[],19],[14,20],[[],21],[14,22],[[],23],[14,24],[7,[[3,[7]]]],[5,[[3,[5]]]],[6,[[3,[6]]]],[8,[[3,[8]]]],[7,[[3,[7]]]],[5,[[3,[5]]]],[6,[[3,[6]]]],[8,[[3,[8]]]],0,0,0,0,0,[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],0,0,[[25,9],10],[[26,9],10],[[25,27]],[[26,27]],[[]],[[[28,[26]]],25],[[[30,[29]]],26],[[]],[29,26],[[[32,[31]]],26],[33,26],[[]],[[]],[[],3],[[],3],[[],3],[[],3],[[],4],[[],4],[34,[[3,[25]]]],0,0,0,[[]],[[]],[[]],[[]],0,0,0,0,0,[[]],[[]],0,0,[35,36],[[36,37],[[3,[38,25]]]],[[]],[[]],0,0,0,0,0,0,[[],3],[[],3],[[],3],[[],3],[[],4],[[],4],[20],[22],[16],[18],[24]],"p":[[3,"DummyClock"],[3,"Timestamp"],[4,"Result"],[3,"TypeId"],[3,"__rtic_internal_blink_MicrosecMono_SpawnHandle"],[3,"__rtic_internal_bme_measure_MicrosecMono_SpawnHandle"],[3,"__rtic_internal_log_nmea_MicrosecMono_SpawnHandle"],[3,"__rtic_internal_sdmmc_log_MicrosecMono_SpawnHandle"],[3,"Formatter"],[6,"Result"],[3,"__rtic_internal_init_Context"],[3,"__rtic_internal_initLocalResources"],[3,"Peripherals"],[3,"Priority"],[3,"__rtic_internal_gps_irqSharedResources"],[3,"__rtic_internal_gps_irq_Context"],[3,"__rtic_internal_log_nmeaSharedResources"],[3,"__rtic_internal_log_nmea_Context"],[3,"__rtic_internal_blinkLocalResources"],[3,"__rtic_internal_blink_Context"],[3,"__rtic_internal_bme_measureLocalResources"],[3,"__rtic_internal_bme_measure_Context"],[3,"__rtic_internal_sdmmc_logLocalResources"],[3,"__rtic_internal_sdmmc_log_Context"],[3,"Report"],[4,"Error"],[3,"Formatter"],[8,"Into"],[4,"Error"],[4,"Error"],[4,"Error"],[4,"Error"],[3,"InvalidConfig"],[15,"str"],[3,"Peripherals"],[3,"Board"],[4,"Option"],[3,"CanSat"],[3,"Local"],[3,"Shared"],[3,"__rtic_internal_Monotonics"],[8,"WrapErr"]]},\
"cansat_test_utils":{"doc":"Test utilities.","t":[0,3,11,11,11,11,11,11,11,11,11],"n":["mock","Serial","borrow","borrow_mut","from","into","new","read","try_from","try_into","type_id"],"q":["cansat_test_utils","cansat_test_utils::mock","","","","","","","","",""],"d":["Mock types.","Mock type implementing <code>embedded_hal::serial</code> traits.","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Creates an instance of serial mock that will output <code>data</code> …","Reads a single byte from the serial.","","",""],"i":[0,0,2,2,2,2,2,2,2,2,2],"f":[0,0,[[]],[[]],[[]],[[]],[1,2],[[[2,[3]]],[[5,[4]]]],[[],6],[[],6],[[],7]],"p":[[8,"IntoIterator"],[3,"Serial"],[8,"Iterator"],[15,"u8"],[6,"Result"],[4,"Result"],[3,"TypeId"]]},\
"xtask":{"doc":"Custom commands for cargo.","t":[13,3,4,13,13,11,11,11,11,11,11,11,11,12,11,11,5,11,11,11,11,11,11,11,11,11,11,5,5,11,11,11,11,11,11,11,11,11,11,5,12,12,12,12],"n":["Build","Cli","Cmd","Embed","Test","augment_args","augment_args_for_update","augment_subcommands","augment_subcommands_for_update","borrow","borrow","borrow_mut","borrow_mut","cmd","command","command_for_update","format_cmd_args","from","from","from_arg_matches","from_arg_matches","from_arg_matches_mut","from_arg_matches_mut","group_id","has_subcommand","into","into","main","run","try_from","try_from","try_into","try_into","type_id","type_id","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches_mut","update_from_arg_matches_mut","workspace_members","args","args","args","pkg_name"],"q":["xtask","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","xtask::Cmd","","",""],"d":["<code>cd</code> and build each package","","","Run <code>cargo embed</code> on a specified package","<code>cd</code> and test each package","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","Arguments for <code>cargo embed</code>","Arguments for <code>cargo build</code>","Arguments for <code>cargo test</code>","Package name"],"i":[8,0,0,8,8,5,5,8,8,5,8,5,8,5,5,5,0,5,8,5,8,5,8,5,8,5,8,0,0,5,8,5,8,5,8,5,8,5,8,0,16,17,18,16],"f":[0,0,0,0,0,[1,1],[1,1],[1,1],[1,1],[[]],[[]],[[]],[[]],0,[[],1],[[],1],[[],[[3,[2]]]],[[]],[[]],[4,[[7,[5,6]]]],[4,[[7,[8,6]]]],[4,[[7,[5,6]]]],[4,[[7,[8,6]]]],[[],[[3,[9]]]],[10,11],[[]],[[]],[[],12],[[],13],[[],7],[[],7],[[],7],[[],7],[[],14],[[],14],[[5,4],[[7,[6]]]],[[8,4],[[7,[6]]]],[[5,4],[[7,[6]]]],[[8,4],[[7,[6]]]],[[],[[13,[[15,[2]]]]]],0,0,0,0],"p":[[3,"Command"],[3,"String"],[4,"Option"],[3,"ArgMatches"],[3,"Cli"],[6,"Error"],[4,"Result"],[4,"Cmd"],[3,"Id"],[15,"str"],[15,"bool"],[3,"ExitCode"],[6,"Result"],[3,"TypeId"],[3,"Vec"],[13,"Embed"],[13,"Build"],[13,"Test"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
