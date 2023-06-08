var searchIndex = JSON.parse('{\
"cansat_core":{"doc":"Platform agnostic library for cansat.","t":"CFFAFAFDMMLLLLLLMMMLMLLLDDDDLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL","n":["Measurements","calculate_altitude","calculate_altitude_with_temperature","measurements","pitch","quantity","roll","Measurements","acceleration","altitude","borrow","borrow_mut","default","format","from","into","nmea","orientation","pressure","serialize","temperature","try_from","try_into","type_id","Angle","Distance","Pressure","Temperature","add","add","add","add","as_celsius","as_degrees","as_hectos","as_kelvins","as_kilos","as_meters","as_pascals","as_radians","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","default","default","default","div","div","div","div","div","div","eq","eq","eq","eq","from","from","from","from","from_celsius","from_degrees","from_hectos","from_kelvins","from_kilos","from_meters","from_pascals","from_radians","into","into","into","into","mul","mul","mul","mul","partial_cmp","partial_cmp","partial_cmp","partial_cmp","sub","sub","sub","sub","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id"],"q":[[0,"cansat_core"],[7,"cansat_core::measurements"],[24,"cansat_core::quantity"]],"d":["","","","","Aircraft pitch rotation.","","Aircraft roll rotation.","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,0,0,0,1,3,2,5,3,5,1,3,2,2,1,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,2,5,1,1,3,2,2,5,1,3,2,5,1,3,2,5,3,5,1,3,2,2,1,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5,1,3,2,5],"f":[0,[1,2],[[3,1],2],0,[[4,4,4],5],0,[[4,4],5],0,0,0,[[]],[[]],[[],6],[[6,7]],[[]],[[]],0,0,0,[[6,8],9],0,[[],9],[[],9],[[],10],0,0,0,0,[[1,1],1],[[3,3],3],[[2,2],2],[[5,5],5],[3,4],[5,4],[1,4],[3,4],[2,4],[2,4],[1,4],[5,4],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,1],[3,3],[2,2],[5,5],[[]],[[]],[[]],[[]],[[],1],[[],2],[[],5],[1,1],[[1,1]],[3,3],[2,2],[[2,2]],[5,5],[[1,1],11],[[3,3],11],[[2,2],11],[[5,5],11],[[]],[[]],[[]],[[]],[4,3],[4,5],[4,1],[4,3],[4,2],[4,2],[4,1],[4,5],[[]],[[]],[[]],[[]],[1,1],[3,3],[2,2],[5,5],[[1,1],[[13,[12]]]],[[3,3],[[13,[12]]]],[[2,2],[[13,[12]]]],[[5,5],[[13,[12]]]],[[1,1],1],[[3,3],3],[[2,2],2],[[5,5],5],[[]],[[]],[[]],[[]],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],10],[[],10],[[],10],[[],10]],"c":[],"p":[[3,"Pressure"],[3,"Distance"],[3,"Temperature"],[15,"f32"],[3,"Angle"],[3,"Measurements"],[3,"Formatter"],[8,"Serializer"],[4,"Result"],[3,"TypeId"],[15,"bool"],[4,"Ordering"],[4,"Option"]]},\
"cansat_gps":{"doc":"Gps device driver using embedded-hal traits.","t":"EDNRNLLLLLLLLLLLLLLLLLLLLLL","n":["Error","Gps","Overflow","STANDARD_MAX_NMEA_LEN","Serial","borrow","borrow","borrow_mut","borrow_mut","fmt","format","from","from","from","from","into","into","last_nmea","new","read_serial","send","try_from","try_from","try_into","try_into","type_id","type_id"],"q":[[0,"cansat_gps"]],"d":["","Gps driver.","","Maximum length of a standard NMEA message including $ and […","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Reads last received NMEA message.","","Reads a single character from serial in a blocking mode …","","","","","","",""],"i":[0,0,2,0,2,8,2,8,2,2,2,8,2,2,2,8,2,8,8,8,8,8,2,8,2,8,2],"f":[0,0,0,0,0,[[]],[[]],[[]],[[]],[[[2,[1]],3],4],[[[2,[5]],6]],[[]],[[]],[7],[[],2],[[]],[[]],[8,[[11,[[10,[9]]]]]],[[],8],[[[8,[12]]],[[13,[2]]]],[[[8,[14]],[15,[9]]],[[13,[2]]]],[[],13],[[],13],[[],13],[[],13],[[],16],[[],16]],"c":[],"p":[[8,"Debug"],[4,"Error"],[3,"Formatter"],[6,"Result"],[8,"Display"],[3,"Formatter"],[15,"never"],[3,"Gps"],[15,"u8"],[3,"Vec"],[4,"Option"],[8,"Read"],[4,"Result"],[8,"Write"],[15,"slice"],[3,"TypeId"]]},\
"cansat_lora":{"doc":"","t":"NNNENNDNNNNNEDENNNLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLMLLLLLLLFLLLLLLLLLLLLLLLLLLL","n":["BadCommand","BadErrorCode","Data","Error","Error","Incomplete","Lora","NoDelimiter","NoPrefix","NoTerminator","Overflow","Parse","ParseError","Response","ResponseContent","Serial","UnclosedErrorParen","Unknown","append","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","content","eq","eq","eq","eq","equivalent","equivalent","equivalent","equivalent","fmt","fmt","fmt","fmt","format","format","from","from","from","from","from","from","from_error_kind","header","into","into","into","into","into","into_serial","new","parse_response","receive","send","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id"],"q":[[0,"cansat_lora"]],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","",""],"i":[2,2,8,0,8,2,0,2,2,2,4,4,0,0,0,4,2,2,2,16,4,7,8,2,16,4,7,8,2,4,2,4,2,7,4,7,8,2,4,7,8,2,4,7,8,2,4,2,16,4,4,7,8,2,2,7,16,4,7,8,2,16,16,0,16,16,4,2,16,4,7,8,2,16,4,7,8,2,16,4,7,8,2],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[1,2],2],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[[4,[3]]],[[4,[3]]]],[2,2],[[]],[[]],0,[[[4,[5]],[4,[5]]],6],[[7,7],6],[[8,8],6],[[2,2],6],[[],6],[[],6],[[],6],[[],6],[[[4,[9]],10],11],[[7,10],11],[[8,10],11],[[2,10],11],[[[4,[9]],12]],[[2,12]],[[]],[[]],[2,4],[[]],[[]],[[]],[1,2],0,[[]],[[]],[[]],[[]],[[]],[[[16,[[0,[[14,[13]],[15,[13]]]]]]],[[0,[[14,[13]],[15,[13]]]]]],[[[0,[[14,[13]],[15,[13]]]]],[[16,[[0,[[14,[13]],[15,[13]]]]]]]],[[[17,[13]]],[[18,[7,2]]]],[[[16,[[0,[[14,[13]],[15,[13]]]]]],[17,[13]]],[[18,[19,4]]]],[[[16,[[0,[[14,[13]],[15,[13]]]]]],[17,[13]]],[[18,[4]]]],[[]],[[]],[[],18],[[],18],[[],18],[[],18],[[],18],[[],18],[[],18],[[],18],[[],18],[[],18],[[],20],[[],20],[[],20],[[],20],[[],20]],"c":[],"p":[[4,"ErrorKind"],[4,"ParseError"],[8,"Clone"],[4,"Error"],[8,"PartialEq"],[15,"bool"],[3,"Response"],[4,"ResponseContent"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[3,"Formatter"],[15,"u8"],[8,"Write"],[8,"Read"],[3,"Lora"],[15,"slice"],[4,"Result"],[15,"usize"],[3,"TypeId"]]},\
"cansat_stm32f4":{"doc":"Binary crate targeting stm32f4 family microcontrollers.","t":"GGGGDGGGGGDGGAMMAMMAAAMFDHDFDDHGDDDDDDDDDDHGALLLLLLLLLLLLLLLLLLLLLLLLLLLLMMMMMMMMLLLLLLLLLLLLLLMMMAMMAFAFLLLLLLLLLLLLLLMMMMMMMMLLLLLMMAMMMAMMMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLDDMMLFDDMLMDDDMMMMMLLMMMDDMMMMLMDDDMMMLLMFNENNLLLLLLLLLLRDLLMLMLLLLLLMLGGGDGGDDGGDGGGGGGRRGGGDMLLLLLLLLLLMMMLLLLLLMMMFFFFFFFFLLLLLMMMMLMMMMMMLLLLLLLLLLLLLLLFFFFFF","n":["Bme280","Bme280Error","Delay","Gps","I2c1Devices","Led","Lis3dh","Lis3dhError","Lora","LoraError","SdLogger","SdmmcController","SdmmcError","app","bme280","controller","error","file","lis3dh","sd_logger","startup","tasks","volume","EXTI1","Local","RTIC_ASYNC_MAX_LOGICAL_PRIO","Shared","USART1","__rtic_internal_blinkLocalResources","__rtic_internal_blink_Context","__rtic_internal_blink_EXEC","__rtic_internal_blink_F","__rtic_internal_gps_irqSharedResources","__rtic_internal_gps_irq_Context","__rtic_internal_idleLocalResources","__rtic_internal_idleSharedResources","__rtic_internal_idle_Context","__rtic_internal_initLocalResources","__rtic_internal_init_Context","__rtic_internal_send_measLocalResources","__rtic_internal_send_measSharedResources","__rtic_internal_send_meas_Context","__rtic_internal_send_meas_EXEC","__rtic_internal_send_meas_F","blink","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","core","cs","csv_record","csv_record","csv_record","delay","delay","device","from","from","from","from","from","from","from","from","from","from","from","from","from","from","gps","gps","gps","gps_irq","i2c1_devices","i2c1_devices","idle","idle","init","init","into","into","into","into","into","into","into","into","into","into","into","into","into","into","led","led","local","local","local","local","lora","lora","new","new","new","new","new","sd_logger","sd_logger","send_meas","shared","shared","shared","shared_resources","statik","tracker","tracker","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","Context","LocalResources","led","local","new","spawn","Context","SharedResources","gps","new","shared","Context","LocalResources","SharedResources","csv_record","delay","gps","i2c1_devices","local","new","new","sd_logger","shared","tracker","Context","LocalResources","core","cs","device","local","new","statik","Context","LocalResources","SharedResources","csv_record","local","lora","new","new","shared","spawn","CriticalDevice","Error","Lora","Response","borrow","borrow_mut","fmt","format","from","from","into","try_from","try_into","type_id","FILENAME","SdLogger","borrow","borrow_mut","controller","drop","file","from","into","new","try_from","try_into","type_id","volume","write","BlockSpi2","Bme280","Bme280Error","Board","Cs2","Delay","Drivers","DummyClock","Gps","GpsError","I2c1Devices","I2c1Proxy","Led","Lis3dh","Lis3dhError","Lora","LoraError","MAX_OPEN_DIRS","MAX_OPEN_FILES","SdmmcController","SdmmcError","Spi2Device","Statik","bme280","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cs2","delay","delay","from","from","from","from","from","get_timestamp","gps","i2c1","i2c1_devices","init","init_bme280","init_board","init_drivers","init_gps","init_lis3dh","init_lora","init_sd_logger","into","into","into","into","into","led","led","lis3dh","lora","new","sd_logger","serial1","serial6","spi2","spi2_device","tracker","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","blink","gps_irq","idle","read_measurements","send_lora_package","send_meas"],"q":[[0,"cansat_stm32f4"],[23,"cansat_stm32f4::app"],[184,"cansat_stm32f4::app::blink"],[190,"cansat_stm32f4::app::gps_irq"],[195,"cansat_stm32f4::app::idle"],[208,"cansat_stm32f4::app::init"],[216,"cansat_stm32f4::app::send_meas"],[226,"cansat_stm32f4::error"],[240,"cansat_stm32f4::sd_logger"],[255,"cansat_stm32f4::startup"],[340,"cansat_stm32f4::tasks"]],"d":["","","","","","","","","","","","","","The RTIC application module","","","","","","","","","","Interrupt handler to dispatch async tasks at priority 1","","Holds the maximum priority level for use by async HAL …","","","Local resources <code>blink</code> has access to","Execution context","","","Shared resources <code>gps_irq</code> has access to","Execution context","Local resources <code>idle</code> has access to","Shared resources <code>idle</code> has access to","Execution context","Local resources <code>init</code> has access to","Execution context","Local resources <code>send_meas</code> has access to","Shared resources <code>send_meas</code> has access to","Execution context","","","Software task","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Core peripherals","Critical section token for init","","","","","","Device peripherals (PAC)","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Hardware task","","","Idle loop","","Initialization function","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Local Resources this task has access to","Local Resources this task has access to","Local Resources this task has access to","Local Resources this task has access to","","","","","","","","","","Software task","Shared Resources this task has access to","Shared Resources this task has access to","Shared Resources this task has access to","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Execution context","Local resources <code>blink</code> has access to","","Local Resources this task has access to","","Spawns the task directly","Execution context","Shared resources <code>gps_irq</code> has access to","","","Shared Resources this task has access to","Execution context","Local resources <code>idle</code> has access to","Shared resources <code>idle</code> has access to","","","","","Local Resources this task has access to","","","","Shared Resources this task has access to","","Execution context","Local resources <code>init</code> has access to","Core peripherals","Critical section token for init","Device peripherals (PAC)","Local Resources this task has access to","","","Execution context","Local resources <code>send_meas</code> has access to","Shared resources <code>send_meas</code> has access to","","Local Resources this task has access to","","","","Shared Resources this task has access to","Spawns the task directly","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Static memory needed for startup.","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","Toggles led every second","USART3 interrupt handler that reads data into the gps …","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,52,22,0,22,52,0,0,0,22,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,53,54,14,3,12,13,1,11,5,15,16,6,10,7,53,54,14,3,12,13,1,11,5,15,16,6,10,7,3,3,53,12,16,54,13,3,53,54,14,3,12,13,1,11,5,15,16,6,10,7,53,12,11,0,54,13,0,0,0,0,53,54,14,3,12,13,1,11,5,15,16,6,10,7,54,10,3,1,6,7,54,15,3,1,5,6,7,54,13,0,1,5,6,0,14,54,13,53,54,14,3,12,13,1,11,5,15,16,6,10,7,53,54,14,3,12,13,1,11,5,15,16,6,10,7,53,54,14,3,12,13,1,11,5,15,16,6,10,7,0,0,10,7,10,0,0,0,11,11,5,0,0,0,12,13,12,13,1,12,13,13,1,13,0,0,3,3,3,3,14,14,0,0,0,16,6,15,15,16,6,0,17,0,17,17,17,17,17,17,17,17,17,17,17,17,0,0,22,22,22,22,22,22,22,22,22,22,22,22,22,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,52,39,52,37,38,28,39,52,37,38,28,37,39,37,39,52,37,38,28,28,39,37,39,0,0,0,0,0,0,0,0,39,52,37,38,28,39,37,52,39,38,39,37,37,37,38,39,39,52,37,38,28,39,52,37,38,28,39,52,37,38,28,0,0,0,0,0,0],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],0,0,0,[[]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,[1,2],0,[3],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,0,[4,3],[[],1],[[],5],[[],6],[[],7],0,0,0,0,0,0,0,0,0,0,[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],0,0,0,0,[[],10],[[],8],0,0,0,[[],11],0,0,0,0,0,0,0,0,0,[[],12],[[],13],0,0,0,0,0,0,0,0,0,[[],14],0,0,0,0,0,0,0,[[],15],[[],16],0,[[],8],0,0,0,0,[[]],[[]],[[17,18],19],[[17,20]],[21,17],[[]],[[]],[[],8],[[],8],[[],9],0,0,[[]],[[]],0,[22],0,[[]],[[]],[23,[[8,[22,24]]]],[[],8],[[],8],[[],9],0,[[22,[26,[25]]],[[8,[27,24]]]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,[[]],[[]],[[]],[[]],[[]],[28,29],0,0,0,[3],[[[32,[[31,[30]]]],33],[[8,[34,35]]]],[36,37],[[37,38],[[8,[39,17]]]],[40,[[8,[41,[43,[42]]]]]],[[[32,[[31,[30]]]]],[[8,[44,45]]]],[46,[[8,[47,17]]]],[[48,[50,[49]],38],[[8,[22,24]]]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,[[],38],0,0,0,0,0,0,[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],9],[[],9],[[],9],[[],9],[[],9],[7],[5],[1,2],[1,51],[[47,[26,[25]]],[[8,[17]]]],[6]],"c":[],"p":[[3,"__rtic_internal_idle_Context"],[15,"never"],[3,"__rtic_internal_init_Context"],[3,"Peripherals"],[3,"__rtic_internal_gps_irq_Context"],[3,"__rtic_internal_send_meas_Context"],[3,"__rtic_internal_blink_Context"],[4,"Result"],[3,"TypeId"],[3,"__rtic_internal_blinkLocalResources"],[3,"__rtic_internal_gps_irqSharedResources"],[3,"__rtic_internal_idleSharedResources"],[3,"__rtic_internal_idleLocalResources"],[3,"__rtic_internal_initLocalResources"],[3,"__rtic_internal_send_measLocalResources"],[3,"__rtic_internal_send_measSharedResources"],[4,"Error"],[3,"Formatter"],[6,"Result"],[3,"Formatter"],[6,"LoraError"],[3,"SdLogger"],[6,"SdmmcController"],[6,"SdmmcError"],[15,"u8"],[15,"slice"],[15,"usize"],[3,"DummyClock"],[3,"Timestamp"],[6,"I2c1"],[3,"AtomicCheckMutex"],[3,"I2cProxy"],[6,"Delay"],[6,"Bme280"],[6,"Bme280Error"],[3,"Peripherals"],[3,"Board"],[3,"Statik"],[3,"Drivers"],[6,"Serial1"],[6,"Gps"],[4,"ErrorKind"],[4,"Error"],[6,"Lis3dh"],[6,"Lis3dhError"],[6,"Serial6"],[6,"Lora"],[6,"Spi2"],[3,"Output"],[6,"PB12"],[3,"Measurements"],[3,"I2c1Devices"],[3,"Shared"],[3,"Local"]]},\
"cansat_test_utils":{"doc":"Test utilities.","t":"ADLLLLLLLMLLMLL","n":["mock","Serial","borrow","borrow_mut","flush","from","into","new","read","rx_data","try_from","try_into","tx_data","type_id","write"],"q":[[0,"cansat_test_utils"],[1,"cansat_test_utils::mock"]],"d":["Mock types.","Mock type implementing <code>embedded_hal::serial</code> traits.","","","Does nothing","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Creates an instance of serial mock that will output <code>data</code> …","Reads a single byte from the serial.","","","","","","Write a single byte to the serial."],"i":[0,0,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[0,0,[[]],[[]],[1,2],[[]],[[]],[3,1],[[[1,[4]]],[[2,[5]]]],0,[[],6],[[],6],0,[[],7],[[1,5],2]],"c":[],"p":[[3,"Serial"],[6,"Result"],[8,"IntoIterator"],[8,"ExactSizeIterator"],[15,"u8"],[4,"Result"],[3,"TypeId"]]},\
"lora_cli":{"doc":"","t":"DEDDNNLLLLLLMLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLLLLFLFFLFFMMFLLLLLLLLLLLLLLLLLLLLLF","n":["Cli","Cmd","Lora","PortArgs","Ports","Receive","augment_args","augment_args","augment_args_for_update","augment_args_for_update","augment_subcommands","augment_subcommands_for_update","baudrate","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cmd","command","command","command_for_update","command_for_update","fmt","fmt","fmt","from","from","from","from","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches_mut","from_arg_matches_mut","from_arg_matches_mut","group_id","group_id","has_subcommand","into","into","into","into","list_ports","listen","lora_error_description","main","new","open_port","parse_lora_error","port","port","receive","receive","send","transmit","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches_mut","update_from_arg_matches_mut","update_from_arg_matches_mut","validate_success_response"],"q":[[0,"lora_cli"]],"d":["","","","","Lists available ports","Start a receive loop","","","","","","","Baudrate","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","Serial port to open","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,5,5,2,6,2,6,5,5,6,15,2,5,6,15,2,5,6,2,2,6,2,6,2,5,6,15,2,5,6,2,5,6,2,5,6,2,6,5,15,2,5,6,0,15,0,0,15,0,0,15,6,0,15,15,15,15,2,5,6,15,2,5,6,15,2,5,6,2,5,6,2,5,6,0],"f":[0,0,0,0,0,0,[1,1],[1,1],[1,1],[1,1],[1,1],[1,1],0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,[[],1],[[],1],[[],1],[[],1],[[2,3],4],[[5,3],4],[[6,3],4],[[]],[[]],[[]],[[]],[7,[[9,[2,8]]]],[7,[[9,[5,8]]]],[7,[[9,[6,8]]]],[7,[[9,[2,8]]]],[7,[[9,[5,8]]]],[7,[[9,[6,8]]]],[[],[[11,[10]]]],[[],[[11,[10]]]],[12,13],[[]],[[]],[[]],[[]],[[],14],[15,[[14,[16]]]],[17,12],[[],14],[[[19,[18]]],15],[6,[[14,[[19,[18]]]]]],[12,[[11,[17]]]],0,0,[6,14],[15,[[14,[20]]]],[[15,[22,[21]]],[[14,[23]]]],[[15,[22,[21]]],[[14,[20]]]],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],24],[[],24],[[],24],[[],24],[[2,7],[[9,[8]]]],[[5,7],[[9,[8]]]],[[6,7],[[9,[8]]]],[[2,7],[[9,[8]]]],[[5,7],[[9,[8]]]],[[6,7],[[9,[8]]]],[12,14]],"c":[],"p":[[3,"Command"],[3,"Cli"],[3,"Formatter"],[6,"Result"],[4,"Cmd"],[3,"PortArgs"],[3,"ArgMatches"],[6,"Error"],[4,"Result"],[3,"Id"],[4,"Option"],[15,"str"],[15,"bool"],[6,"Result"],[3,"Lora"],[8,"Iterator"],[15,"i32"],[8,"SerialPort"],[3,"Box"],[3,"String"],[15,"u8"],[15,"slice"],[15,"usize"],[3,"TypeId"]]},\
"xtask":{"doc":"Custom commands for cargo.","t":"NDENNLLLLLLLLFMLLFFLLLLLLLLLLFFFLLLLLLLLLLFMMMMMM","n":["Build","Cli","Cmd","Embed","Test","augment_args","augment_args_for_update","augment_subcommands","augment_subcommands_for_update","borrow","borrow","borrow_mut","borrow_mut","build","cmd","command","command_for_update","embed","format_cmd","from","from","from_arg_matches","from_arg_matches","from_arg_matches_mut","from_arg_matches_mut","group_id","has_subcommand","into","into","main","run","test","try_from","try_from","try_into","try_into","type_id","type_id","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches_mut","update_from_arg_matches_mut","workspace_members","args","args","args","package","package","package"],"q":[[0,"xtask"],[43,"xtask::Cmd"]],"d":["<code>cd</code> into each package and run <code>cargo build</code>","","","<code>cd</code> into specified or default package and run <code>cargo embed</code>","<code>cd</code> into each package and run <code>cargo test</code>","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","Arguments for <code>cargo embed</code>","Arguments for <code>cargo build</code>","Arguments for <code>cargo test</code>","Name of the package","Name of the package","Name of the package"],"i":[13,0,0,13,13,10,10,13,13,10,13,10,13,0,10,10,10,0,0,10,13,10,13,10,13,10,13,10,13,0,0,0,10,13,10,13,10,13,10,13,10,13,0,19,20,21,19,20,21],"f":[0,0,0,0,0,[1,1],[1,1],[1,1],[1,1],[[]],[[]],[[]],[[]],[[[3,[2]],[5,[4]]],6],0,[[],1],[[],1],[[[3,[2]],[5,[4]]],6],[[7,[8,[4]]],4],[[]],[[]],[9,[[12,[10,11]]]],[9,[[12,[13,11]]]],[9,[[12,[10,11]]]],[9,[[12,[13,11]]]],[[],[[3,[14]]]],[7,15],[[]],[[]],[[],16],[[],6],[[[3,[2]],[5,[4]]],6],[[],12],[[],12],[[],12],[[],12],[[],17],[[],17],[[10,9],[[12,[11]]]],[[13,9],[[12,[11]]]],[[10,9],[[12,[11]]]],[[13,9],[[12,[11]]]],[[],[[6,[[5,[18]]]]]],0,0,0,0,0,0],"c":[],"p":[[3,"Command"],[3,"OsString"],[4,"Option"],[3,"String"],[3,"Vec"],[6,"Result"],[15,"str"],[15,"slice"],[3,"ArgMatches"],[3,"Cli"],[6,"Error"],[4,"Result"],[4,"Cmd"],[3,"Id"],[15,"bool"],[3,"ExitCode"],[3,"TypeId"],[3,"PathBuf"],[13,"Embed"],[13,"Build"],[13,"Test"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
