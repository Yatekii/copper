EESchema Schematic File Version 3
LIBS:PSU-rescue
LIBS:power
LIBS:device
LIBS:transistors
LIBS:conn
LIBS:linear
LIBS:regul
LIBS:74xx
LIBS:cmos4000
LIBS:adc-dac
LIBS:memory
LIBS:xilinx
LIBS:microcontrollers
LIBS:dsp
LIBS:microchip
LIBS:analog_switches
LIBS:motorola
LIBS:texas
LIBS:intel
LIBS:audio
LIBS:interface
LIBS:digital-audio
LIBS:philips
LIBS:display
LIBS:cypress
LIBS:siliconi
LIBS:opto
LIBS:atmel
LIBS:contrib
LIBS:valves
LIBS:nordic
LIBS:standard_components
LIBS:atmel_bastli
LIBS:itead
LIBS:connectors
LIBS:advanced_monolithic
LIBS:NDK
LIBS:axsem
LIBS:bourns
LIBS:buydisplay
LIBS:cirrus
LIBS:cui
LIBS:fairchild
LIBS:linear_tech
LIBS:micrel
LIBS:onsemi
LIBS:wurth
LIBS:antennas
LIBS:PSU-cache
EELAYER 26 0
EELAYER END
$Descr A3 16535 11693
encoding utf-8
Sheet 1 1
Title "PSU"
Date "2017-10-05"
Rev "V2"
Comp "Noah Huesser / yatekii@yatekii.ch"
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
Text Notes 8050 10900 0    276  Italic 55
Mesh Node\nr3 autumn 2017\nby yatekii
$Comp
L R R15
U 1 1 55030D28
P 14100 1100
F 0 "R15" V 14180 1100 50  0000 C CNN
F 1 "470" V 14100 1100 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 14030 1100 30  0001 C CNN
F 3 "" H 14100 1100 30  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-07470RL/311-470GRCT-ND/729738" V 14100 1100 60  0001 C CNN "Supplier"
	1    14100 1100
	0    1    1    0   
$EndComp
Text Label 15250 1100 2    60   ~ 0
LED1
$Comp
L R R16
U 1 1 55F93662
P 14100 1500
F 0 "R16" V 14180 1500 50  0000 C CNN
F 1 "470" V 14100 1500 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 14030 1500 30  0001 C CNN
F 3 "" H 14100 1500 30  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-07470RL/311-470GRCT-ND/729738" V 14100 1500 60  0001 C CNN "Supplier"
	1    14100 1500
	0    1    1    0   
$EndComp
Text Label 15250 1500 2    60   ~ 0
LED2
$Comp
L SW_PUSH SW1
U 1 1 55F9D378
P 14350 1950
F 0 "SW1" H 14500 2060 50  0000 C CNN
F 1 "SW_PUSH" H 14350 1870 50  0000 C CNN
F 2 "Buttons_Switches_SMD:SW_SPST_B3U-1000P" H 14350 1950 60  0001 C CNN
F 3 "" H 14350 1950 60  0000 C CNN
F 4 "https://www.digikey.ch/product-detail/de/omron-electronics-inc-emc-div/B3U-1000P/SW1020CT-ND/1534357" H 1050 1200 60  0001 C CNN "Supplier"
	1    14350 1950
	1    0    0    -1  
$EndComp
Text Label 15250 1950 2    60   ~ 0
BUTTON1
$Comp
L SW_PUSH SW2
U 1 1 55FAACF3
P 14400 2400
F 0 "SW2" H 14550 2510 50  0000 C CNN
F 1 "SW_PUSH" H 14400 2320 50  0000 C CNN
F 2 "Buttons_Switches_SMD:SW_SPST_B3U-1000P" H 14400 2400 60  0001 C CNN
F 3 "" H 14400 2400 60  0000 C CNN
F 4 "https://www.digikey.ch/product-detail/de/omron-electronics-inc-emc-div/B3U-1000P/SW1020CT-ND/1534357" H 1050 1200 60  0001 C CNN "Supplier"
	1    14400 2400
	1    0    0    -1  
$EndComp
Text Label 15250 2400 2    60   ~ 0
RESET
$Comp
L ATM90E26 U1
U 1 1 57A7641D
P 2850 5050
F 0 "U1" H 2850 6237 60  0000 C CNN
F 1 "ATM90E26" H 2850 6131 60  0000 C CNN
F 2 "atmel:M90E26" H 3000 5350 60  0001 C CNN
F 3 "" H 3000 5350 60  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/microchip-technology/ATM90E26-YU-R/ATM90E26-YU-RCT-ND/4899919" H 1050 1200 60  0001 C CNN "Supplier"
	1    2850 5050
	1    0    0    -1  
$EndComp
$Comp
L TRANSFO T1
U 1 1 57A7650D
P 6750 4350
F 0 "T1" H 6650 4600 50  0000 C CNN
F 1 "TRANSFO" V 6600 4350 50  0000 C CNN
F 2 "triad:CSE187L" H 6750 4350 50  0001 C CNN
F 3 "" H 6750 4350 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/triad-magnetics/CSE187L/237-1103-ND/242546" H 1050 1200 60  0001 C CNN "Supplier"
	1    6750 4350
	1    0    0    -1  
$EndComp
Text Notes 6600 4700 0    79   ~ 0
1:500
$Comp
L +3V3 #PWR01
U 1 1 57A8E880
P 1700 4000
F 0 "#PWR01" H 1700 3850 50  0001 C CNN
F 1 "+3V3" H 1715 4173 50  0000 C CNN
F 2 "" H 1700 4000 50  0000 C CNN
F 3 "" H 1700 4000 50  0000 C CNN
	1    1700 4000
	1    0    0    -1  
$EndComp
$Comp
L R R1
U 1 1 57A8ECCB
P 1700 4300
F 0 "R1" H 1770 4346 50  0000 L CNN
F 1 "0" V 1700 4250 50  0000 L CNN
F 2 "Resistors_SMD:R_0603" V 1630 4300 50  0001 C CNN
F 3 "" H 1700 4300 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/tdk-corporation/MPZ1608S300ATAH0/445-1562-1-ND/571892" H 1700 4300 60  0001 C CNN "Supplier"
	1    1700 4300
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR02
U 1 1 57A8F21A
P 2200 4250
F 0 "#PWR02" H 2200 4000 50  0001 C CNN
F 1 "GND" V 2205 4122 50  0000 R CNN
F 2 "" H 2200 4250 50  0000 C CNN
F 3 "" H 2200 4250 50  0000 C CNN
	1    2200 4250
	0    1    1    0   
$EndComp
$Comp
L GND #PWR03
U 1 1 57A8F408
P 2100 4550
F 0 "#PWR03" H 2100 4300 50  0001 C CNN
F 1 "GND" V 2105 4422 50  0000 R CNN
F 2 "" H 2100 4550 50  0000 C CNN
F 3 "" H 2100 4550 50  0000 C CNN
	1    2100 4550
	0    1    1    0   
$EndComp
$Comp
L C C3
U 1 1 57A8F675
P 1150 4300
F 0 "C3" H 1050 4400 50  0000 L CNN
F 1 "100n" H 950 4200 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 1188 4150 50  0001 C CNN
F 3 "" H 1150 4300 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL05B104KO5NNNC/1276-1001-1-ND/3889087" H 1150 4300 60  0001 C CNN "Supplier"
	1    1150 4300
	1    0    0    -1  
$EndComp
$Comp
L C C1
U 1 1 57A8F761
P 850 4300
F 0 "C1" H 750 4400 50  0000 L CNN
F 1 "10u" H 700 4200 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805" H 888 4150 50  0001 C CNN
F 3 "" H 850 4300 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM21BR60J106ME19L/490-1718-1-ND/587425" H 850 4300 60  0001 C CNN "Supplier"
	1    850  4300
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR04
U 1 1 57A8FC0A
P 1150 4550
F 0 "#PWR04" H 1150 4300 50  0001 C CNN
F 1 "GND" H 1155 4377 50  0000 C CNN
F 2 "" H 1150 4550 50  0000 C CNN
F 3 "" H 1150 4550 50  0000 C CNN
	1    1150 4550
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR05
U 1 1 57A90090
P 1150 5100
F 0 "#PWR05" H 1150 4850 50  0001 C CNN
F 1 "GND" H 1155 4927 50  0000 C CNN
F 2 "" H 1150 5100 50  0000 C CNN
F 3 "" H 1150 5100 50  0000 C CNN
	1    1150 5100
	1    0    0    -1  
$EndComp
$Comp
L C C5
U 1 1 57A9030F
P 1450 4450
F 0 "C5" V 1400 4550 50  0000 C CNN
F 1 "100n" V 1600 4450 50  0000 C CNN
F 2 "Capacitors_SMD:C_0402" H 1488 4300 50  0001 C CNN
F 3 "" H 1450 4450 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL05B104KO5NNNC/1276-1001-1-ND/3889087" H 1050 1200 60  0001 C CNN "Supplier"
	1    1450 4450
	0    1    1    0   
$EndComp
$Comp
L C C4
U 1 1 57A9120B
P 1150 4950
F 0 "C4" H 1200 5050 50  0000 L CNN
F 1 "100n" H 1200 4850 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 1188 4800 50  0001 C CNN
F 3 "" H 1150 4950 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL05B104KO5NNNC/1276-1001-1-ND/3889087" H 1050 1200 60  0001 C CNN "Supplier"
	1    1150 4950
	1    0    0    -1  
$EndComp
$Comp
L C C2
U 1 1 57A91851
P 850 4950
F 0 "C2" H 900 5050 50  0000 L CNN
F 1 "1u" H 900 4850 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 888 4800 50  0001 C CNN
F 3 "" H 850 4950 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL10B105KP8NNNC/1276-1946-1-ND/3890032" H 1050 1200 60  0001 C CNN "Supplier"
	1    850  4950
	1    0    0    -1  
$EndComp
Text Label 1550 5100 0    79   ~ 0
RST_METER
Text Label 2900 1650 2    79   ~ 0
I_OUT
Text Label 5700 4150 0    79   ~ 0
I_IN
Text Label 5700 4550 0    79   ~ 0
I_OUT
$Comp
L R R12
U 1 1 57A95844
P 7150 4350
F 0 "R12" H 7220 4396 50  0000 L CNN
F 1 "18" V 7150 4300 50  0000 L CNN
F 2 "Resistors_SMD:R_0603" V 7080 4350 50  0001 C CNN
F 3 "" H 7150 4350 50  0000 C CNN
F 4 "https://www.digikey.ch/product-detail/de/yageo/RC0603JR-0718RL/311-18GRCT-ND/729671" H 1050 1200 60  0001 C CNN "Supplier"
	1    7150 4350
	1    0    0    -1  
$EndComp
$Comp
L R R13
U 1 1 57A95B16
P 7450 3950
F 0 "R13" V 7350 3950 50  0000 C CNN
F 1 "100" V 7450 3950 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 7380 3950 50  0001 C CNN
F 3 "" H 7450 3950 50  0000 C CNN
F 4 "https://www.digikey.ch/product-detail/de/yageo/RC0603FR-07100RL/311-100HRCT-ND/729835" H 1050 1200 60  0001 C CNN "Supplier"
	1    7450 3950
	0    1    1    0   
$EndComp
$Comp
L R R14
U 1 1 57A95C4E
P 7450 4750
F 0 "R14" V 7350 4750 50  0000 C CNN
F 1 "100" V 7450 4750 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 7380 4750 50  0001 C CNN
F 3 "" H 7450 4750 50  0000 C CNN
F 4 "https://www.digikey.ch/product-detail/de/yageo/RC0603FR-07100RL/311-100HRCT-ND/729835" H 1050 1200 60  0001 C CNN "Supplier"
	1    7450 4750
	0    1    1    0   
$EndComp
$Comp
L C C25
U 1 1 57A9670C
P 7750 4200
F 0 "C25" H 7865 4246 50  0000 L CNN
F 1 "330n" H 7865 4155 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 7788 4050 50  0001 C CNN
F 3 "" H 7750 4200 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM188R71C334KA01D/490-3294-1-ND/702835" H 1050 1200 60  0001 C CNN "Supplier"
	1    7750 4200
	1    0    0    -1  
$EndComp
$Comp
L C C26
U 1 1 57A9681E
P 7750 4500
F 0 "C26" H 7865 4546 50  0000 L CNN
F 1 "330n" H 7865 4455 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 7788 4350 50  0001 C CNN
F 3 "" H 7750 4500 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM188R71C334KA01D/490-3294-1-ND/702835" H 1050 1200 60  0001 C CNN "Supplier"
	1    7750 4500
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR06
U 1 1 57A971C8
P 8150 4350
F 0 "#PWR06" H 8150 4100 50  0001 C CNN
F 1 "GND" V 8155 4222 50  0000 R CNN
F 2 "" H 8150 4350 50  0000 C CNN
F 3 "" H 8150 4350 50  0000 C CNN
	1    8150 4350
	0    -1   -1   0   
$EndComp
Text Label 2000 5650 0    79   ~ 0
I1P
Text Label 2000 5750 0    79   ~ 0
I1N
Text Label 8000 4750 2    79   ~ 0
I1N_
Text Label 8000 3950 2    79   ~ 0
I1P_
Text Label 2000 5300 0    79   ~ 0
VP
Text Label 2000 5400 0    79   ~ 0
VN
Text Label 1250 2800 0    79   ~ 0
N
$Comp
L R R3
U 1 1 57A9B1F6
P 6350 5150
F 0 "R3" V 6250 5050 50  0000 L CNN
F 1 "402k" V 6350 5050 50  0000 L CNN
F 2 "Resistors_SMD:R_0805" V 6280 5150 50  0001 C CNN
F 3 "" H 6350 5150 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0805FR-07402KL/311-402KCRCT-ND/730890" H 1050 1200 60  0001 C CNN "Supplier"
	1    6350 5150
	1    0    0    -1  
$EndComp
$Comp
L R R4
U 1 1 57A9BB85
P 6350 5550
F 0 "R4" V 6250 5450 50  0000 L CNN
F 1 "402k" V 6350 5450 50  0000 L CNN
F 2 "Resistors_SMD:R_0805" V 6280 5550 50  0001 C CNN
F 3 "" H 6350 5550 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0805FR-07402KL/311-402KCRCT-ND/730890" H 1050 1200 60  0001 C CNN "Supplier"
	1    6350 5550
	1    0    0    -1  
$EndComp
$Comp
L R R5
U 1 1 57A9BD12
P 6350 5950
F 0 "R5" V 6250 5850 50  0000 L CNN
F 1 "2.1k" V 6350 5850 50  0000 L CNN
F 2 "Resistors_SMD:R_0603" V 6280 5950 50  0001 C CNN
F 3 "" H 6350 5950 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603FR-072K1L/311-2.10KHRCT-ND/729960" H 1050 1200 60  0001 C CNN "Supplier"
	1    6350 5950
	1    0    0    -1  
$EndComp
$Comp
L C C21
U 1 1 57A9BF91
P 6550 5950
F 0 "C21" H 6600 6050 50  0000 L CNN
F 1 "33n" H 6600 5850 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 6588 5800 50  0001 C CNN
F 3 "" H 6550 5950 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL10F333ZB8NNNC/1276-2369-1-ND/3890455" H 1050 1200 60  0001 C CNN "Supplier"
	1    6550 5950
	1    0    0    -1  
$EndComp
Text Label 5700 5000 0    79   ~ 0
LIVE
Text Label 5700 6100 0    79   ~ 0
N
Text Label 6550 5800 2    79   ~ 0
VP_
$Comp
L C C24
U 1 1 57A9D94B
P 7350 5950
F 0 "C24" H 7350 6050 50  0000 L CNN
F 1 "33n" H 7400 5850 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 7388 5800 50  0001 C CNN
F 3 "" H 7350 5950 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL10F333ZB8NNNC/1276-2369-1-ND/3890455" H 1050 1200 60  0001 C CNN "Supplier"
	1    7350 5950
	1    0    0    -1  
$EndComp
$Comp
L R R11
U 1 1 57A9DA71
P 7050 5950
F 0 "R11" H 7100 6050 50  0000 L CNN
F 1 "2.1k" V 7050 5850 50  0000 L CNN
F 2 "Resistors_SMD:R_0603" V 6980 5950 50  0001 C CNN
F 3 "" H 7050 5950 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603FR-072K1L/311-2.10KHRCT-ND/729960" H 1050 1200 60  0001 C CNN "Supplier"
	1    7050 5950
	1    0    0    -1  
$EndComp
Text Label 7100 5800 0    79   ~ 0
VN
Text Label 2000 5950 0    79   ~ 0
I2P
Text Label 2000 6050 0    79   ~ 0
I2N
Text Notes 8350 6250 0    79   ~ 0
Sensing
Text Notes 5500 5700 1    79   ~ 0
Voltage
Text Notes 5500 4500 1    79   ~ 0
Current
$Comp
L CONN_01X01 TP3
U 1 1 57AB17E9
P 1800 5650
F 0 "TP3" H 2100 5650 50  0000 C CNN
F 1 "I1P" H 1925 5650 50  0000 C CNN
F 2 "general:testpoint" H 1800 5650 60  0001 C CNN
F 3 "" H 1800 5650 60  0000 C CNN
	1    1800 5650
	-1   0    0    1   
$EndComp
$Comp
L CONN_01X01 TP4
U 1 1 57AB1D7B
P 1800 5750
F 0 "TP4" H 2100 5750 50  0000 C CNN
F 1 "I1N" H 1925 5750 50  0000 C CNN
F 2 "general:testpoint" H 1800 5750 60  0001 C CNN
F 3 "" H 1800 5750 60  0000 C CNN
	1    1800 5750
	-1   0    0    1   
$EndComp
$Comp
L CONN_01X01 TP2
U 1 1 57AB3FC4
P 1800 5300
F 0 "TP2" H 2100 5300 50  0000 C CNN
F 1 "VP" H 1925 5300 50  0000 C CNN
F 2 "general:testpoint" H 1800 5300 60  0001 C CNN
F 3 "" H 1800 5300 60  0000 C CNN
	1    1800 5300
	-1   0    0    1   
$EndComp
Text Notes 6250 3100 0    79   ~ 0
Relay for static power on
Text Notes 650  3100 0    79   ~ 0
Triac for switched power on
Text Notes 1650 4300 2    28   Italic 0
Ferrite bead ->
$Comp
L Crystal_Small Y2
U 1 1 57AC30E7
P 4400 4700
F 0 "Y2" H 4300 4800 50  0000 C CNN
F 1 "8.192MHz" H 4400 4600 50  0000 C CNN
F 2 "Crystals:ABM3" H 4400 4700 60  0001 C CNN
F 3 "" H 4400 4700 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/abracon-llc/ABM3B-8.000MHZ-B2-T/535-9720-1-ND/1873254" H 1050 1200 60  0001 C CNN "Supplier"
	1    4400 4700
	0    1    1    0   
$EndComp
$Comp
L C C18
U 1 1 57AC3658
P 4650 4500
F 0 "C18" V 4500 4500 50  0000 C CNN
F 1 "12p" V 4800 4500 50  0000 C CNN
F 2 "Capacitors_SMD:C_0402" H 4688 4350 50  0001 C CNN
F 3 "" H 4650 4500 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/kemet/C0402C120J5GACTU/399-1013-1-ND/411288" H 1050 1200 60  0001 C CNN "Supplier"
	1    4650 4500
	0    1    1    0   
$EndComp
$Comp
L C C19
U 1 1 57AC37F1
P 4650 4900
F 0 "C19" V 4500 4900 50  0000 C CNN
F 1 "12p" V 4800 4900 50  0000 C CNN
F 2 "Capacitors_SMD:C_0402" H 4688 4750 50  0001 C CNN
F 3 "" H 4650 4900 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/kemet/C0402C120J5GACTU/399-1013-1-ND/411288" H 1050 1200 60  0001 C CNN "Supplier"
	1    4650 4900
	0    1    1    0   
$EndComp
$Comp
L GND #PWR07
U 1 1 57AC6230
P 4950 4700
F 0 "#PWR07" H 4950 4450 50  0001 C CNN
F 1 "GND" V 4955 4572 50  0000 R CNN
F 2 "" H 4950 4700 50  0000 C CNN
F 3 "" H 4950 4700 50  0000 C CNN
	1    4950 4700
	0    -1   -1   0   
$EndComp
Text Label 3750 4150 2    79   ~ 0
CS
Text Label 3750 4250 2    79   ~ 0
SCK
Text Label 3750 4450 2    79   ~ 0
MOSI
Text Label 3750 4350 2    79   ~ 0
MISO
Text Label 4050 4950 2    79   ~ 0
CF1
Text Label 4050 5050 2    79   ~ 0
CF2
Text Label 4050 5250 2    79   ~ 0
IRQ
Text Label 4050 5350 2    79   ~ 0
WARNOUT
Text Label 4050 5450 2    79   ~ 0
ZX
Text Label 4050 5650 2    79   ~ 0
MMD0
Text Label 4050 5750 2    79   ~ 0
MMD1
$Comp
L GND #PWR08
U 1 1 57AC8CC4
P 4050 6050
F 0 "#PWR08" H 4050 5800 50  0001 C CNN
F 1 "GND" H 4055 5877 50  0000 C CNN
F 2 "" H 4050 6050 50  0000 C CNN
F 3 "" H 4050 6050 50  0000 C CNN
	1    4050 6050
	1    0    0    -1  
$EndComp
Text Notes 14900 3100 0    79   ~ 0
LEDs & Buttons
Text Notes 11800 9900 0    60   ~ 0
Big Thank You to RuuviTag for giving me a nice reference and the nRF component!
$Comp
L +3V3 #PWR09
U 1 1 57AF2757
P 13800 1100
F 0 "#PWR09" H 13800 950 50  0001 C CNN
F 1 "+3V3" H 13815 1273 50  0000 C CNN
F 2 "" H 13800 1100 50  0000 C CNN
F 3 "" H 13800 1100 50  0000 C CNN
	1    13800 1100
	0    -1   -1   0   
$EndComp
$Comp
L +3V3 #PWR010
U 1 1 57AF2A7D
P 13800 1500
F 0 "#PWR010" H 13800 1350 50  0001 C CNN
F 1 "+3V3" H 13815 1673 50  0000 C CNN
F 2 "" H 13800 1500 50  0000 C CNN
F 3 "" H 13800 1500 50  0000 C CNN
	1    13800 1500
	0    -1   -1   0   
$EndComp
Text Label 3850 2100 2    79   ~ 0
T_ON_
$Comp
L TRIAC U2
U 1 1 57A7BA5D
P 2100 2200
F 0 "U2" H 2428 2229 50  0000 L CNN
F 1 "TRIAC" H 2428 2320 50  0000 L CNN
F 2 "Power_Integrations:TO-220" H 2100 2200 50  0001 C CNN
F 3 "" H 2100 2200 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/stmicroelectronics/BTA16-600CRG/497-3388-5-ND/669145" H 1050 1200 60  0001 C CNN "Supplier"
	1    2100 2200
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR012
U 1 1 581BE555
P 3200 2800
F 0 "#PWR012" H 3200 2550 50  0001 C CNN
F 1 "GND" V 3205 2672 50  0000 R CNN
F 2 "" H 3200 2800 50  0000 C CNN
F 3 "" H 3200 2800 50  0000 C CNN
	1    3200 2800
	1    0    0    -1  
$EndComp
$Comp
L PB134005 K1
U 1 1 59D287DC
P 6600 1400
F 0 "K1" H 6600 1797 60  0000 C CNN
F 1 "PB134005" H 6600 1691 60  0000 C CNN
F 2 "TEconnectivity:TE134005" H 6600 1400 60  0001 C CNN
F 3 "" H 6600 1400 60  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/te-connectivity-potter-brumfield-relays/PB134005/PB766-ND/813832" H 1050 1200 60  0001 C CNN "Supplier"
	1    6600 1400
	1    0    0    -1  
$EndComp
$Comp
L D D3
U 1 1 59D29EE4
P 5650 1350
F 0 "D3" V 5604 1428 50  0000 L CNN
F 1 "1N4148" V 5695 1428 50  0000 L CNN
F 2 "Diodes_SMD:D_SOD-123" H 5650 1350 50  0001 C CNN
F 3 "" H 5650 1350 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/smc-diode-solutions/1N4148WTR/1655-1360-1-ND/6022805" H 1050 1200 60  0001 C CNN "Supplier"
	1    5650 1350
	0    1    1    0   
$EndComp
Text Label 7300 1250 2    79   ~ 0
I_OUT
Text Label 7300 1550 2    79   ~ 0
N
$Comp
L R R9
U 1 1 59D2C0C8
P 3200 2600
F 0 "R9" V 3100 2550 50  0000 L CNN
F 1 "10k" V 3200 2550 50  0000 L CNN
F 2 "Resistors_SMD:R_0603" V 3130 2600 50  0001 C CNN
F 3 "" H 3200 2600 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-0710KL/311-10KGRCT-ND/729647" H 1050 1200 60  0001 C CNN "Supplier"
	1    3200 2600
	1    0    0    1   
$EndComp
$Comp
L R R8
U 1 1 59D2E996
P 2850 2400
F 0 "R8" V 2750 2350 50  0000 L CNN
F 1 "TBD" V 2850 2350 50  0000 L CNN
F 2 "Resistors_SMD:R_0603" V 2780 2400 50  0001 C CNN
F 3 "" H 2850 2400 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-0710KL/311-10KGRCT-ND/729647" H 1050 1200 60  0001 C CNN "Supplier"
	1    2850 2400
	0    -1   1    0   
$EndComp
Text Label 5600 1900 0    79   ~ 0
R_ON_
$Comp
L GND #PWR014
U 1 1 59D31E03
P 6250 2400
F 0 "#PWR014" H 6250 2150 50  0001 C CNN
F 1 "GND" V 6255 2272 50  0000 R CNN
F 2 "" H 6250 2400 50  0000 C CNN
F 3 "" H 6250 2400 50  0000 C CNN
	1    6250 2400
	-1   0    0    -1  
$EndComp
$Comp
L BSS138-RESCUE-PSU Q3
U 1 1 59D31E10
P 6150 1800
F 0 "Q3" H 6341 1846 50  0000 L CNN
F 1 "BSS138" H 6341 1755 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-23" H 6350 1725 50  0001 L CIN
F 3 "" H 6150 1800 50  0000 L CNN
F 4 "https://www.digikey.com/product-detail/en/on-semiconductor/BSS138/BSS138CT-ND/244294" H 1050 1200 60  0001 C CNN "Supplier"
	1    6150 1800
	1    0    0    -1  
$EndComp
Text Notes 15700 9450 0    60   ~ 0
PSU
$Comp
L D D5
U 1 1 59D63254
P 10950 7500
F 0 "D5" H 10950 7715 50  0000 C CNN
F 1 "S115FP" H 10950 7624 50  0000 C CNN
F 2 "Diodes_SMD:D_SOD-123" H 10950 7500 50  0001 C CNN
F 3 "" H 10950 7500 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/on-semiconductor/S115FP/S115FPCT-ND/5892124" H 1050 1200 60  0001 C CNN "Supplier"
	1    10950 7500
	1    0    0    -1  
$EndComp
$Comp
L D D6
U 1 1 59D63466
P 12150 7300
F 0 "D6" H 12150 7515 50  0000 C CNN
F 1 "MUR160S" H 12150 7424 50  0000 C CNN
F 2 "Diodes_SMD:D_SMB" H 12150 7300 50  0001 C CNN
F 3 "" H 12150 7300 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/taiwan-semiconductor-corporation/MUR160S-R5G/MUR160S-R5GCT-ND/7357857" H 12150 7300 60  0001 C CNN "Supplier"
	1    12150 7300
	1    0    0    -1  
$EndComp
$Comp
L D D7
U 1 1 59D635AE
P 13750 7100
F 0 "D7" H 13750 6885 50  0000 C CNN
F 1 "MBRS140T3G" H 13750 6976 50  0000 C CNN
F 2 "Diodes_SMD:D_SMB" H 13750 7100 50  0001 C CNN
F 3 "" H 13750 7100 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/on-semiconductor/MBRS140T3G/MBRS140T3GOSCT-ND/918002" H 1050 1200 60  0001 C CNN "Supplier"
	1    13750 7100
	-1   0    0    1   
$EndComp
$Comp
L GND #PWR015
U 1 1 59D637B2
P 11100 9300
F 0 "#PWR015" H 11100 9050 50  0001 C CNN
F 1 "GND" H 11105 9127 50  0000 C CNN
F 2 "" H 11100 9300 50  0000 C CNN
F 3 "" H 11100 9300 50  0000 C CNN
	1    11100 9300
	1    0    0    -1  
$EndComp
$Comp
L CP C30
U 1 1 59D673E7
P 9750 7150
F 0 "C30" H 9868 7196 50  0000 L CNN
F 1 "4.7" H 9868 7105 50  0000 L CNN
F 2 "Capacitors_THT:CP_Radial_D8.0mm_P3.50mm" H 9788 7000 50  0001 C CNN
F 3 "" H 9750 7150 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/wurth-electronics-inc/860011374005/732-8689-1-ND/5728642" H 1050 1200 60  0001 C CNN "Supplier"
	1    9750 7150
	1    0    0    -1  
$EndComp
$Comp
L CP C31
U 1 1 59D69684
P 10100 8900
F 0 "C31" H 10218 8946 50  0000 L CNN
F 1 "2.2u" H 10218 8855 50  0000 L CNN
F 2 "Capacitors_SMD:C_1206" H 10138 8750 50  0001 C CNN
F 3 "" H 10100 8900 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL31B225KBHNFNE/1276-3137-1-ND/3891223" H 1050 1200 60  0001 C CNN "Supplier"
	1    10100 8900
	1    0    0    -1  
$EndComp
$Comp
L C C33
U 1 1 59D68BB7
P 11100 8900
F 0 "C33" H 11215 8946 50  0000 L CNN
F 1 "10nF" H 11215 8855 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 11138 8750 50  0001 C CNN
F 3 "" H 11100 8900 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM033R71A103KA01D/490-3194-1-ND/702735" H 1050 1200 60  0001 C CNN "Supplier"
	1    11100 8900
	1    0    0    -1  
$EndComp
Text Label 8100 8000 0    60   ~ 0
N
Text Label 8100 7100 0    60   ~ 0
LIVE
$Comp
L C C35
U 1 1 59D69174
P 13700 8600
F 0 "C35" H 13815 8646 50  0000 L CNN
F 1 "100n" H 13815 8555 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 13738 8450 50  0001 C CNN
F 3 "" H 13700 8600 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL05B104KO5NNNC/1276-1001-1-ND/3889087" H 1050 1200 60  0001 C CNN "Supplier"
	1    13700 8600
	0    1    1    0   
$EndComp
$Comp
L R R22
U 1 1 59D6A390
P 10100 7700
F 0 "R22" V 10000 7700 50  0000 C CNN
F 1 "R" V 10100 7700 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 10030 7700 50  0001 C CNN
F 3 "" H 10100 7700 50  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-0710KL/311-10KGRCT-ND/729647" H 1050 1200 60  0001 C CNN "Supplier"
	1    10100 7700
	-1   0    0    1   
$EndComp
$Comp
L R R23
U 1 1 59D6A553
P 11800 6800
F 0 "R23" V 11880 6800 50  0000 C CNN
F 1 "R" V 11800 6800 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 11730 6800 50  0001 C CNN
F 3 "" H 11800 6800 50  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-0710KL/311-10KGRCT-ND/729647" H 1050 1200 60  0001 C CNN "Supplier"
	1    11800 6800
	-1   0    0    1   
$EndComp
$Comp
L R R24
U 1 1 59D6A713
P 14300 8050
F 0 "R24" V 14380 8050 50  0000 C CNN
F 1 "R" V 14300 8050 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 14230 8050 50  0001 C CNN
F 3 "" H 14300 8050 50  0001 C CNN
F 4 "" H 1050 1200 60  0001 C CNN "Supplier"
	1    14300 8050
	-1   0    0    1   
$EndComp
$Comp
L R R25
U 1 1 59D6A8BA
P 14900 8050
F 0 "R25" V 14980 8050 50  0000 C CNN
F 1 "R" V 14900 8050 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 14830 8050 50  0001 C CNN
F 3 "" H 14900 8050 50  0001 C CNN
F 4 "" H 1050 1200 60  0001 C CNN "Supplier"
	1    14900 8050
	-1   0    0    1   
$EndComp
$Comp
L R R26
U 1 1 59D6AA16
P 14900 8950
F 0 "R26" V 14980 8950 50  0000 C CNN
F 1 "R" V 14900 8950 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 14830 8950 50  0001 C CNN
F 3 "" H 14900 8950 50  0001 C CNN
F 4 "" H 1050 1200 60  0001 C CNN "Supplier"
	1    14900 8950
	-1   0    0    1   
$EndComp
$Comp
L C C34
U 1 1 59D6C1F6
P 11400 6950
F 0 "C34" H 11515 6996 50  0000 L CNN
F 1 "C" H 11515 6905 50  0000 L CNN
F 2 "Capacitors_SMD:C_1206" H 11438 6800 50  0001 C CNN
F 3 "" H 11400 6950 50  0000 C CNN
F 4 "" H 1050 1200 60  0001 C CNN "Supplier"
	1    11400 6950
	1    0    0    -1  
$EndComp
$Comp
L LTV-817 U7
U 1 1 59D6D7C0
P 13000 8500
F 0 "U7" H 12800 8700 50  0000 L CNN
F 1 "SFH615A" H 13000 8700 50  0000 L CNN
F 2 "ISOCOM:SFH615A" H 12800 8300 50  0001 L CIN
F 3 "" H 13000 8400 50  0001 L CNN
F 4 "https://www.digikey.com/product-detail/en/isocom-components-2004-ltd/SFH615A-4XSMT-R/SFH615A-4XSMCT-ND/5037113" H 1050 1200 60  0001 C CNN "Supplier"
	1    13000 8500
	-1   0    0    -1  
$EndComp
$Comp
L +5V #PWR016
U 1 1 59D7139F
P 14900 6650
F 0 "#PWR016" H 14900 6500 50  0001 C CNN
F 1 "+5V" H 14900 6790 50  0000 C CNN
F 2 "" H 14900 6650 50  0001 C CNN
F 3 "" H 14900 6650 50  0001 C CNN
	1    14900 6650
	1    0    0    -1  
$EndComp
$Comp
L 7508110151 T2
U 1 1 59D81DBB
P 13050 7300
F 0 "T2" H 13050 7947 60  0000 C CNN
F 1 "7508110151" H 13050 7841 60  0000 C CNN
F 2 "wurth:7508110151" H 13050 7300 60  0001 C CNN
F 3 "" H 13050 7300 60  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/wurth-electronics-midcom/7508110151/1297-1122-ND/4800054" H 1050 1200 60  0001 C CNN "Supplier"
	1    13050 7300
	1    0    0    -1  
$EndComp
NoConn ~ 12700 7100
$Comp
L NCP431 U8
U 1 1 59D8336B
P 13450 8950
F 0 "U8" H 13578 9028 60  0000 L CNN
F 1 "NCP431" H 13578 8922 60  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-23" H 13450 8900 60  0001 C CNN
F 3 "" H 13450 8900 60  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/on-semiconductor/NCP431AVSNT1G/NCP431AVSNT1GOSCT-ND/5969048" H 1050 1200 60  0001 C CNN "Supplier"
	1    13450 8950
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR017
U 1 1 59D88602
P 12050 8050
F 0 "#PWR017" H 12050 7800 50  0001 C CNN
F 1 "GND" H 12055 7877 50  0000 C CNN
F 2 "" H 12050 8050 50  0000 C CNN
F 3 "" H 12050 8050 50  0000 C CNN
	1    12050 8050
	1    0    0    -1  
$EndComp
Text Notes 15500 7200 0    60   ~ 0
16V
Text Notes 10150 8750 0    60   ~ 0
36V
Text Notes 9750 7300 0    60   ~ 0
400V
Text Notes 11400 6850 0    60   ~ 0
400V
$Comp
L D_Bridge_+-AA D4
U 1 1 59D8FCA5
P 9000 7550
F 0 "D4" H 9050 7825 50  0000 L CNN
F 1 "D_Bridge_+-AA" H 9050 7750 50  0000 L CNN
F 2 "onsemi:Micro-DIP" H 9000 7550 50  0001 C CNN
F 3 "" H 9000 7550 50  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/fairchild-on-semiconductor/MDB8S/MDB8SFSCT-ND/3137113" H 1050 1200 60  0001 C CNN "Supplier"
	1    9000 7550
	1    0    0    -1  
$EndComp
$Comp
L R R20
U 1 1 59D926D7
P 11800 7100
F 0 "R20" V 11880 7100 50  0000 C CNN
F 1 "R" V 11800 7100 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 11730 7100 50  0001 C CNN
F 3 "" H 11800 7100 50  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-0710KL/311-10KGRCT-ND/729647" H 1050 1200 60  0001 C CNN "Supplier"
	1    11800 7100
	-1   0    0    1   
$EndComp
Text Notes 11950 6900 1    60   ~ 0
300V
Text Notes 11950 7200 1    60   ~ 0
300V
$Comp
L LED-RESCUE-PSU D1
U 1 1 59D93B6D
P 14600 1100
F 0 "D1" H 14592 845 50  0000 C CNN
F 1 "LED" H 14592 936 50  0000 C CNN
F 2 "LEDs:LED_0603" H 14600 1100 50  0001 C CNN
F 3 "" H 14600 1100 50  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/lite-on-inc/LTST-C193TBKT-5A/160-1827-1-ND/2355044" H 1050 1200 60  0001 C CNN "Supplier"
	1    14600 1100
	-1   0    0    1   
$EndComp
$Comp
L LED-RESCUE-PSU D2
U 1 1 59D93D77
P 14600 1500
F 0 "D2" H 14592 1245 50  0000 C CNN
F 1 "LED" H 14592 1336 50  0000 C CNN
F 2 "LEDs:LED_0603" H 14600 1500 50  0001 C CNN
F 3 "" H 14600 1500 50  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/lite-on-inc/LTST-C193TBKT-5A/160-1827-1-ND/2355044" H 1050 1200 60  0001 C CNN "Supplier"
	1    14600 1500
	-1   0    0    1   
$EndComp
Text Notes 12000 3100 0    79   ~ 0
Power Supply
$Comp
L +3V3 #PWR018
U 1 1 57AAAC37
P 12350 1800
F 0 "#PWR018" H 12350 1650 50  0001 C CNN
F 1 "+3V3" H 12365 1973 50  0000 C CNN
F 2 "" H 12350 1800 50  0000 C CNN
F 3 "" H 12350 1800 50  0000 C CNN
	1    12350 1800
	1    0    0    -1  
$EndComp
$Comp
L L_Small L4
U 1 1 57AAA67A
P 11950 1800
F 0 "L4" V 12135 1800 50  0000 C CNN
F 1 "4.7u" V 12044 1800 50  0000 C CNN
F 2 "wurth:WE-GF-Typ-L" H 11950 1800 50  0001 C CNN
F 3 "" H 11950 1800 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/wurth-electronics-inc/744764904/732-7294-1-ND/5353274" H 1050 1200 60  0001 C CNN "Supplier"
	1    11950 1800
	0    -1   -1   0   
$EndComp
$Comp
L GND #PWR019
U 1 1 57AA9BA5
P 10850 2300
F 0 "#PWR019" H 10850 2050 50  0001 C CNN
F 1 "GND" H 10855 2127 50  0000 C CNN
F 2 "" H 10850 2300 50  0000 C CNN
F 3 "" H 10850 2300 50  0000 C CNN
	1    10850 2300
	1    0    0    -1  
$EndComp
$Comp
L C C29
U 1 1 57AA962D
P 11550 1950
F 0 "C29" H 11665 1996 50  0000 L CNN
F 1 "100n" H 11665 1905 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 11588 1800 50  0001 C CNN
F 3 "" H 11550 1950 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL05B104KO5NNNC/1276-1001-1-ND/3889087" H 1050 1200 60  0001 C CNN "Supplier"
	1    11550 1950
	1    0    0    -1  
$EndComp
$Comp
L AMS1117 U6
U 1 1 57AA8115
P 10850 1900
F 0 "U6" H 10875 2331 79  0000 C CNN
F 1 "AMS1117" H 10875 2196 79  0000 C CNN
F 2 "TO_SOT_Packages_SMD:SOT-223" H 10850 1900 79  0001 C CNN
F 3 "" H 10850 1900 79  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/diodes-incorporated/AP2114H-3.3TRG1/AP2114H-3.3TRG1DICT-ND/4505142" H 1050 1200 60  0001 C CNN "Supplier"
	1    10850 1900
	1    0    0    -1  
$EndComp
$Comp
L C C27
U 1 1 57AA6EE7
P 9900 1950
F 0 "C27" H 10015 1996 50  0000 L CNN
F 1 "330n" H 10015 1905 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 9938 1800 50  0001 C CNN
F 3 "" H 9900 1950 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM188R71C334KA01D/490-3294-1-ND/702835" H 1050 1200 60  0001 C CNN "Supplier"
	1    9900 1950
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X01 TP5
U 1 1 56978685
P 9900 1600
F 0 "TP5" H 10200 1600 50  0000 C CNN
F 1 "5V" H 10025 1600 50  0000 C CNN
F 2 "general:testpoint" H 9900 1600 60  0001 C CNN
F 3 "" H 9900 1600 60  0000 C CNN
	1    9900 1600
	0    -1   -1   0   
$EndComp
$Comp
L CONN_01X01 TP6
U 1 1 568F6874
P 11550 1600
F 0 "TP6" H 11850 1600 50  0000 C CNN
F 1 "3V3" H 11700 1600 50  0000 C CNN
F 2 "general:testpoint" H 11550 1600 60  0001 C CNN
F 3 "" H 11550 1600 60  0000 C CNN
	1    11550 1600
	0    -1   -1   0   
$EndComp
$Comp
L Chip_Antenna A1
U 1 1 581A0E22
P 12150 3750
F 0 "A1" V 12203 3622 60  0000 R CNN
F 1 "Chip_Antenna" V 12097 3622 60  0000 R CNN
F 2 "johansontechnology:2450AT43A100" H 12250 3750 60  0001 C CNN
F 3 "" H 12250 3750 60  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/johanson-technology-inc/2450AT43A100E/712-1009-1-ND/1560838" H 1050 1200 60  0001 C CNN "Supplier"
	1    12150 3750
	0    -1   -1   0   
$EndComp
$Comp
L GND #PWR020
U 1 1 57B0D881
P 14550 4450
F 0 "#PWR020" H 14550 4200 50  0001 C CNN
F 1 "GND" H 14555 4277 50  0000 C CNN
F 2 "" H 14550 4450 50  0000 C CNN
F 3 "" H 14550 4450 50  0000 C CNN
	1    14550 4450
	0    1    1    0   
$EndComp
Text Label 14550 4750 0    60   ~ 0
SWDIO
Text Label 14550 4650 0    60   ~ 0
SWCLK
Text Label 14550 4550 0    60   ~ 0
RESET
$Comp
L +3V3 #PWR021
U 1 1 57B0D19A
P 14550 4850
F 0 "#PWR021" H 14550 4700 50  0001 C CNN
F 1 "+3V3" H 14565 5023 50  0000 C CNN
F 2 "" H 14550 4850 50  0000 C CNN
F 3 "" H 14550 4850 50  0000 C CNN
	1    14550 4850
	0    -1   -1   0   
$EndComp
$Comp
L CONN_01X05 P11
U 1 1 57B0C699
P 15250 4650
F 0 "P11" H 15200 4950 50  0000 L CNN
F 1 "SWD" V 15350 4600 50  0000 L CNN
F 2 "Connectors_Molex:Molex_PicoBlade_53398-0571" H 15250 4650 50  0001 C CNN
F 3 "" H 15250 4650 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/molex-llc/0533980571/WM7609CT-ND/699083" H 1050 1200 60  0001 C CNN "Supplier"
	1    15250 4650
	1    0    0    -1  
$EndComp
Text Notes 15050 6250 0    79   ~ 0
SWD Interface
Text Notes 12900 6250 0    79   ~ 0
BLE Antenna
Text Notes 11400 4575 0    28   Italic 0
Shunt capacitor or inductor. For convenience, a shunt capacitor is preferred.\nA shunt can be removed without changing the rest of the circuit and a\ncapacitor is cheaper than an inductor.
$Comp
L GND #PWR022
U 1 1 55FBEE73
P 11100 4700
F 0 "#PWR022" H 11100 4450 60  0001 C CNN
F 1 "GND" H 11100 4550 60  0000 C CNN
F 2 "" H 11100 4700 60  0000 C CNN
F 3 "" H 11100 4700 60  0000 C CNN
	1    11100 4700
	1    0    0    -1  
$EndComp
$Comp
L C C28
U 1 1 55FBEE6D
P 11100 4500
F 0 "C28" H 11125 4600 50  0000 L CNN
F 1 "TBD" H 11150 4400 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 11138 4350 30  0001 C CNN
F 3 "" H 11100 4500 60  0000 C CNN
F 4 "" H 1050 1200 60  0001 C CNN "Supplier"
	1    11100 4500
	1    0    0    -1  
$EndComp
Text Label 10050 4200 2    60   ~ 0
ANTENNA
NoConn ~ 5700 765089
Text Label 6100 7750 2    60   ~ 0
R_ON
NoConn ~ 3500 8750
NoConn ~ 3500 9350
Text Label 2700 8850 0    60   ~ 0
WARNOUT
NoConn ~ 5700 8650
NoConn ~ 5700 8550
NoConn ~ 5700 8450
NoConn ~ 5700 9250
NoConn ~ 5700 9950
Text Label 6250 9150 2    60   ~ 0
LED2
NoConn ~ 5700 8050
$Comp
L GND #PWR023
U 1 1 57ACFF46
P 6600 9650
F 0 "#PWR023" H 6600 9400 50  0001 C CNN
F 1 "GND" V 6605 9522 50  0000 R CNN
F 2 "" H 6600 9650 50  0000 C CNN
F 3 "" H 6600 9650 50  0000 C CNN
	1    6600 9650
	0    -1   -1   0   
$EndComp
$Comp
L R R17
U 1 1 57ACFF40
P 6400 9650
F 0 "R17" V 6300 9650 50  0000 C CNN
F 1 "10k" V 6400 9650 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 6330 9650 50  0001 C CNN
F 3 "" H 6400 9650 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-0710KL/311-10KGRCT-ND/729647" H 1050 1200 60  0001 C CNN "Supplier"
	1    6400 9650
	0    1    1    0   
$EndComp
Text Label 6250 9750 2    60   ~ 0
T_ON
$Comp
L R R7
U 1 1 57AAF0BD
P 6400 9450
F 0 "R7" V 6300 9450 50  0000 C CNN
F 1 "10k" V 6400 9450 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 6330 9450 50  0001 C CNN
F 3 "" H 6400 9450 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-0710KL/311-10KGRCT-ND/729647" H 1050 1200 60  0001 C CNN "Supplier"
	1    6400 9450
	0    1    1    0   
$EndComp
$Comp
L GND #PWR024
U 1 1 57AAE85C
P 6600 9450
F 0 "#PWR024" H 6600 9200 50  0001 C CNN
F 1 "GND" V 6605 9322 50  0000 R CNN
F 2 "" H 6600 9450 50  0000 C CNN
F 3 "" H 6600 9450 50  0000 C CNN
	1    6600 9450
	0    -1   -1   0   
$EndComp
Text Label 6250 8950 2    60   ~ 0
RESET
$Comp
L nRF52832 U3
U 1 1 558C64D6
P 4600 8550
F 0 "U3" H 3750 7000 60  0000 C CNN
F 1 "nRF52832" H 5250 7000 60  0000 C CNN
F 2 "Housings_DFN_QFN:UQFN-48-1EP_6x6mm_Pitch0.4mm" H 4600 9050 60  0001 C CNN
F 3 "" H 4600 9050 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/nordic-semiconductor-asa/NRF52832-QFAB-R/1490-1055-1-ND/6051567" H 1050 1200 60  0001 C CNN "Supplier"
	1    4600 8550
	1    0    0    -1  
$EndComp
Text Label 6250 9450 2    60   ~ 0
RST_METER
Text Label 6250 9050 2    60   ~ 0
MMD1
Text Label 6100 8150 2    60   ~ 0
MMD0
Text Label 2700 9050 0    60   ~ 0
CF2
Text Label 2700 8950 0    60   ~ 0
CF1
Text Label 2700 9150 0    60   ~ 0
IRQ
Text Label 2700 9250 0    60   ~ 0
ZX
$Comp
L GND #PWR025
U 1 1 57A8FFB1
P 3750 10500
F 0 "#PWR025" H 3750 10250 60  0001 C CNN
F 1 "GND" V 3750 10300 60  0000 C CNN
F 2 "" H 3750 10500 60  0000 C CNN
F 3 "" H 3750 10500 60  0000 C CNN
	1    3750 10500
	0    1    1    0   
$EndComp
$Comp
L NX3225SA Y3
U 1 1 57A8C2FB
P 4600 10600
F 0 "Y3" H 4600 10947 60  0000 C CNN
F 1 "NX3225SA" H 4600 10841 60  0000 C CNN
F 2 "NDK:NX3225SA" H 4600 10600 60  0001 C CNN
F 3 "" H 4600 10600 60  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/ndk-america-inc/NX3225SA-12.000M-STD-CRS-2/644-1128-1-ND/1788483" H 1050 1200 60  0001 C CNN "Supplier"
	1    4600 10600
	1    0    0    -1  
$EndComp
Text Notes 1100 9800 1    60   ~ 0
NFC Antenna
$Comp
L L_Small L2
U 1 1 57AE8335
P 2500 8050
F 0 "L2" V 2550 7950 50  0000 C CNN
F 1 "10u" V 2550 8100 50  0000 C CNN
F 2 "Capacitors_SMD:C_0603" H 2500 8050 50  0001 C CNN
F 3 "" H 2500 8050 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM21BR60J106ME19L/490-1718-1-ND/587425" H 1050 1200 60  0001 C CNN "Supplier"
	1    2500 8050
	0    1    1    0   
$EndComp
$Comp
L L_Small L1
U 1 1 57AE820D
P 2500 7950
F 0 "L1" V 2550 8050 50  0000 C CNN
F 1 "15n" V 2550 7900 50  0000 C CNN
F 2 "Capacitors_SMD:C_0603" H 2500 7950 50  0001 C CNN
F 3 "" H 2500 7950 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CIH10T15NJNC/1276-6308-1-ND/3972226" H 1050 1200 60  0001 C CNN "Supplier"
	1    2500 7950
	0    -1   -1   0   
$EndComp
$Comp
L +3V3 #PWR026
U 1 1 57AE77CD
P 3450 7150
F 0 "#PWR026" H 3450 7000 50  0001 C CNN
F 1 "+3V3" H 3465 7323 50  0000 C CNN
F 2 "" H 3450 7150 50  0000 C CNN
F 3 "" H 3450 7150 50  0000 C CNN
	1    3450 7150
	1    0    0    -1  
$EndComp
Text Notes 6700 11050 0    60   ~ 0
Controller / BLE Smart
$Comp
L L_Small L3
U 1 1 57ABE6F3
P 7000 7250
F 0 "L3" V 7050 7250 50  0000 C CNN
F 1 "3n9" V 6950 7250 50  0000 C CNN
F 2 "Capacitors_SMD:C_0603" H 7000 7250 50  0001 C CNN
F 3 "" H 7000 7250 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CIH10T3N9SNC/1276-6303-1-ND/3972221" H 1050 1200 60  0001 C CNN "Supplier"
	1    7000 7250
	0    -1   -1   0   
$EndComp
$Comp
L CONN_01X01 P6
U 1 1 56976D9A
P 6300 8250
F 0 "P6" H 6600 8250 50  0000 C CNN
F 1 "TP" H 6425 8250 50  0000 C CNN
F 2 "general:testpoint" H 6300 8250 60  0001 C CNN
F 3 "" H 6300 8250 60  0000 C CNN
	1    6300 8250
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X01 P5
U 1 1 56976D07
P 6300 7950
F 0 "P5" H 6300 7850 50  0000 C CNN
F 1 "TP" H 6425 7950 50  0000 C CNN
F 2 "general:testpoint" H 6300 7950 60  0001 C CNN
F 3 "" H 6300 7950 60  0000 C CNN
	1    6300 7950
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X01 P4
U 1 1 56976B39
P 6300 7850
F 0 "P4" H 6300 7950 50  0000 C CNN
F 1 "TP" H 6425 7850 50  0000 C CNN
F 2 "general:testpoint" H 6300 7850 60  0001 C CNN
F 3 "" H 6300 7850 60  0000 C CNN
	1    6300 7850
	1    0    0    -1  
$EndComp
Text Notes 6500 7025 0    28   Italic 0
nRF52 side matching
$Comp
L C C22
U 1 1 5694EE23
P 6750 7450
F 0 "C22" H 6775 7550 50  0000 L CNN
F 1 "0p8" H 6775 7350 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 6788 7300 30  0001 C CNN
F 3 "" H 6750 7450 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM1555C1HR80BA01D/490-6269-1-ND/3845466" H 1050 1200 60  0001 C CNN "Supplier"
	1    6750 7450
	1    0    0    -1  
$EndComp
Text Label 6250 9850 2    60   ~ 0
CS
Text Label 6250 9650 2    60   ~ 0
BUTTON1
NoConn ~ 5700 9550
Text Label 6250 9350 2    60   ~ 0
LED1
$Comp
L GND #PWR027
U 1 1 558EF937
P 6750 7700
F 0 "#PWR027" H 6750 7450 60  0001 C CNN
F 1 "GND" H 6750 7550 60  0000 C CNN
F 2 "" H 6750 7700 60  0000 C CNN
F 3 "" H 6750 7700 60  0000 C CNN
	1    6750 7700
	1    0    0    -1  
$EndComp
Text Notes 2675 8500 0    28   Italic 0
XTAL SMD 3215, 32.768 kHz, 9 pF, ±20 ppm\nCapacitors, NP0, ±2%
Text Notes 4050 10900 0    28   Italic 0
XTAL SMD 2016, 32 MHz, Cl=8 pF, Total Tol: ±40 ppm\nCapacitors, NP0, ±2%
Text Notes 2300 8100 2    28   Italic 0
High frequency chip inductor ±10% ->\n\n\n\nChip inductor, IDC,min = 50 mA, ±20% ->
Text Notes 2025 10600 2    28   Italic 0
Battery protection!\n\nIf the antenna is exposed to a strong NFC field\ncurrent may flow in the opposite direction on the\nsupply due to parasitic diodes and ESD structures.\n\nIf the battery used does not tolerate return current,\na series diode must be placed between the battery\nand the device in order to protect the battery.
$Comp
L GND #PWR029
U 1 1 558E8B8F
P 2250 9850
F 0 "#PWR029" H 2250 9600 60  0001 C CNN
F 1 "GND" H 2250 9700 60  0000 C CNN
F 2 "" H 2250 9850 60  0000 C CNN
F 3 "" H 2250 9850 60  0000 C CNN
	1    2250 9850
	1    0    0    -1  
$EndComp
$Comp
L C C9
U 1 1 558E8A9B
P 2250 9700
F 0 "C9" H 2275 9800 50  0000 L CNN
F 1 "TBD" H 2275 9600 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 2288 9550 30  0001 C CNN
F 3 "" H 2250 9700 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM1555C1HR80BA01D/490-6269-1-ND/3845466" H 1050 1200 60  0001 C CNN "Supplier"
	1    2250 9700
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR030
U 1 1 558D5038
P 3750 10700
F 0 "#PWR030" H 3750 10450 60  0001 C CNN
F 1 "GND" V 3750 10500 60  0000 C CNN
F 2 "" H 3750 10700 60  0000 C CNN
F 3 "" H 3750 10700 60  0000 C CNN
	1    3750 10700
	0    1    1    0   
$EndComp
$Comp
L GND #PWR031
U 1 1 558D4C39
P 5450 10500
F 0 "#PWR031" H 5450 10250 60  0001 C CNN
F 1 "GND" V 5450 10300 60  0000 C CNN
F 2 "" H 5450 10500 60  0000 C CNN
F 3 "" H 5450 10500 60  0000 C CNN
	1    5450 10500
	0    -1   -1   0   
$EndComp
$Comp
L GND #PWR032
U 1 1 558D4B5C
P 5450 10700
F 0 "#PWR032" H 5450 10450 60  0001 C CNN
F 1 "GND" V 5450 10500 60  0000 C CNN
F 2 "" H 5450 10700 60  0000 C CNN
F 3 "" H 5450 10700 60  0000 C CNN
	1    5450 10700
	0    -1   -1   0   
$EndComp
$Comp
L C C17
U 1 1 558D465C
P 3900 10700
F 0 "C17" H 3925 10800 50  0000 L CNN
F 1 "12p" H 3925 10600 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3938 10550 30  0001 C CNN
F 3 "" H 3900 10700 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM1555C1H120JA01D/490-5924-1-ND/3721281" H 1050 1200 60  0001 C CNN "Supplier"
	1    3900 10700
	0    -1   -1   0   
$EndComp
Text Label 5150 10500 2    60   ~ 0
XC2
Text Label 4250 10700 2    60   ~ 0
XC1
Text Label 3350 8350 0    60   ~ 0
XC2
Text Label 3350 8250 0    60   ~ 0
XC1
$Comp
L C C20
U 1 1 558D2342
P 5300 10500
F 0 "C20" H 5325 10600 50  0000 L CNN
F 1 "12p" H 5325 10400 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 5338 10350 30  0001 C CNN
F 3 "" H 5300 10500 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM1555C1H120JA01D/490-5924-1-ND/3721281" H 1050 1200 60  0001 C CNN "Supplier"
	1    5300 10500
	0    1    1    0   
$EndComp
$Comp
L GND #PWR033
U 1 1 558D0661
P 2100 8500
F 0 "#PWR033" H 2100 8250 60  0001 C CNN
F 1 "GND" V 2100 8300 60  0000 C CNN
F 2 "" H 2100 8500 60  0000 C CNN
F 3 "" H 2100 8500 60  0000 C CNN
	1    2100 8500
	0    1    1    0   
$EndComp
$Comp
L GND #PWR034
U 1 1 558D058A
P 2100 8700
F 0 "#PWR034" H 2100 8450 60  0001 C CNN
F 1 "GND" V 2100 8500 60  0000 C CNN
F 2 "" H 2100 8700 60  0000 C CNN
F 3 "" H 2100 8700 60  0000 C CNN
	1    2100 8700
	0    1    1    0   
$EndComp
$Comp
L C C7
U 1 1 558D004A
P 2250 8700
F 0 "C7" V 2300 8550 50  0000 L CNN
F 1 "12p" V 2300 8750 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 2288 8550 30  0001 C CNN
F 3 "" H 2250 8700 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM1555C1H120JA01D/490-5924-1-ND/3721281" H 1050 1200 60  0001 C CNN "Supplier"
	1    2250 8700
	0    -1   -1   0   
$EndComp
$Comp
L C C6
U 1 1 558CFF4D
P 2250 8500
F 0 "C6" V 2300 8350 50  0000 L CNN
F 1 "12p" V 2300 8550 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 2288 8350 30  0001 C CNN
F 3 "" H 2250 8500 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM1555C1H120JA01D/490-5924-1-ND/3721281" H 1050 1200 60  0001 C CNN "Supplier"
	1    2250 8500
	0    -1   -1   0   
$EndComp
$Comp
L C C10
U 1 1 558C97A2
P 3050 8200
F 0 "C10" V 3100 8050 50  0000 L CNN
F 1 "1u" V 3000 8050 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 3088 8050 30  0001 C CNN
F 3 "" H 3050 8200 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL10B105KP8NNNC/1276-1946-1-ND/3890032" H 1050 1200 60  0001 C CNN "Supplier"
	1    3050 8200
	0    1    1    0   
$EndComp
$Comp
L GND #PWR035
U 1 1 558CDC13
P 3000 7450
F 0 "#PWR035" H 3000 7200 60  0001 C CNN
F 1 "GND" V 3000 7250 60  0000 C CNN
F 2 "" H 3000 7450 60  0000 C CNN
F 3 "" H 3000 7450 60  0000 C CNN
	1    3000 7450
	0    1    1    0   
$EndComp
$Comp
L GND #PWR036
U 1 1 558CDC0D
P 3000 7350
F 0 "#PWR036" H 3000 7100 60  0001 C CNN
F 1 "GND" V 3000 7150 60  0000 C CNN
F 2 "" H 3000 7350 60  0000 C CNN
F 3 "" H 3000 7350 60  0000 C CNN
	1    3000 7350
	0    1    1    0   
$EndComp
$Comp
L GND #PWR037
U 1 1 558CDC07
P 3000 7250
F 0 "#PWR037" H 3000 7000 60  0001 C CNN
F 1 "GND" V 3000 7050 60  0000 C CNN
F 2 "" H 3000 7250 60  0000 C CNN
F 3 "" H 3000 7250 60  0000 C CNN
	1    3000 7250
	0    1    1    0   
$EndComp
$Comp
L GND #PWR038
U 1 1 558CC7F9
P 2850 8200
F 0 "#PWR038" H 2850 7950 60  0001 C CNN
F 1 "GND" V 2850 8000 60  0000 C CNN
F 2 "" H 2850 8200 60  0000 C CNN
F 3 "" H 2850 8200 60  0000 C CNN
	1    2850 8200
	0    1    1    0   
$EndComp
$Comp
L GND #PWR039
U 1 1 558CA466
P 3000 7850
F 0 "#PWR039" H 3000 7600 60  0001 C CNN
F 1 "GND" V 3000 7650 60  0000 C CNN
F 2 "" H 3000 7850 60  0000 C CNN
F 3 "" H 3000 7850 60  0000 C CNN
	1    3000 7850
	0    1    1    0   
$EndComp
$Comp
L GND #PWR040
U 1 1 558CA398
P 3000 7750
F 0 "#PWR040" H 3000 7500 60  0001 C CNN
F 1 "GND" V 3000 7550 60  0000 C CNN
F 2 "" H 3000 7750 60  0000 C CNN
F 3 "" H 3000 7750 60  0000 C CNN
	1    3000 7750
	0    1    1    0   
$EndComp
$Comp
L C C13
U 1 1 558C9D2D
P 3200 7450
F 0 "C13" V 3250 7250 50  0000 L CNN
F 1 "4u7" V 3250 7500 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3238 7300 30  0001 C CNN
F 3 "" H 3200 7450 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL05A475MQ5NQNC/1276-1483-1-ND/3889569" H 1050 1200 60  0001 C CNN "Supplier"
	1    3200 7450
	0    -1   -1   0   
$EndComp
$Comp
L C C12
U 1 1 558C9C5B
P 3200 7350
F 0 "C12" V 3250 7200 50  0000 L CNN
F 1 "100n" V 3250 7400 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3238 7200 30  0001 C CNN
F 3 "" H 3200 7350 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL05B104KO5NNNC/1276-1001-1-ND/3889087" H 1050 1200 60  0001 C CNN "Supplier"
	1    3200 7350
	0    -1   -1   0   
$EndComp
$Comp
L C C11
U 1 1 558C9B80
P 3200 7250
F 0 "C11" V 3250 7100 50  0000 L CNN
F 1 "100n" V 3250 7300 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3238 7100 30  0001 C CNN
F 3 "" H 3200 7250 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL05B104KO5NNNC/1276-1001-1-ND/3889087" H 1050 1200 60  0001 C CNN "Supplier"
	1    3200 7250
	0    -1   -1   0   
$EndComp
$Comp
L C C16
U 1 1 558C96D0
P 3200 7850
F 0 "C16" V 3250 7650 50  0000 L CNN
F 1 "100p" V 3250 7900 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3238 7700 30  0001 C CNN
F 3 "" H 3200 7850 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/CC0402JRNPO9BN101/311-1024-1-ND/302941" H 1050 1200 60  0001 C CNN "Supplier"
	1    3200 7850
	0    -1   -1   0   
$EndComp
$Comp
L C C15
U 1 1 558C9601
P 3200 7750
F 0 "C15" V 3250 7550 50  0000 L CNN
F 1 "NA" V 3250 7800 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3238 7600 30  0001 C CNN
F 3 "" H 3200 7750 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/CC0402JRNPO9BN101/311-1024-1-ND/302941" H 1050 1200 60  0001 C CNN "Supplier"
	1    3200 7750
	0    -1   -1   0   
$EndComp
$Comp
L GND #PWR041
U 1 1 558C93B4
P 3000 7650
F 0 "#PWR041" H 3000 7400 60  0001 C CNN
F 1 "GND" V 3000 7450 60  0000 C CNN
F 2 "" H 3000 7650 60  0000 C CNN
F 3 "" H 3000 7650 60  0000 C CNN
	1    3000 7650
	0    1    1    0   
$EndComp
$Comp
L C C14
U 1 1 558C8F56
P 3200 7650
F 0 "C14" V 3250 7450 50  0000 L CNN
F 1 "100n" V 3250 7700 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3238 7500 30  0001 C CNN
F 3 "" H 3200 7650 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL05B104KO5NNNC/1276-1001-1-ND/3889087" H 1050 1200 60  0001 C CNN "Supplier"
	1    3200 7650
	0    -1   -1   0   
$EndComp
$Comp
L GND #PWR042
U 1 1 558C7E29
P 3400 10100
F 0 "#PWR042" H 3400 9850 60  0001 C CNN
F 1 "GND" H 3400 9950 60  0000 C CNN
F 2 "" H 3400 10100 60  0000 C CNN
F 3 "" H 3400 10100 60  0000 C CNN
	1    3400 10100
	1    0    0    -1  
$EndComp
$Comp
L Crystal_Small Y1
U 1 1 558C6F32
P 2500 8600
F 0 "Y1" V 2675 8600 50  0000 C CNN
F 1 "32K" V 2350 8600 50  0000 C CNN
F 2 "kyocera:ST3215SB" H 2500 8600 60  0001 C CNN
F 3 "" H 2500 8600 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/avx-corp-kyocera-corp/ST3215SB32768H5HPWAA/478-5428-1-ND/1991720" H 1050 1200 60  0001 C CNN "Supplier"
	1    2500 8600
	0    1    1    0   
$EndComp
Text Label 6250 8850 2    60   ~ 0
SWCLK
Text Label 6250 8750 2    60   ~ 0
SWDIO
Text Label 7650 7250 2    60   ~ 0
ANTENNA
Text Label 6100 7850 2    60   ~ 0
SCK
Text Label 6100 7950 2    60   ~ 0
MISO
Text Label 6100 8250 2    60   ~ 0
MOSI
Text Notes 13350 7400 1    60   ~ 0
15.42
Text Notes 12650 7650 0    60   ~ 0
4.91
$Comp
L NCP107x U4
U 1 1 59FB1DF9
P 11600 8000
F 0 "U4" H 11450 8200 60  0000 C CNN
F 1 "NCP107x" H 11600 7800 60  0000 C CNN
F 2 "TO_SOT_Packages_SMD:SOT-223" H 11600 8000 60  0001 C CNN
F 3 "" H 11600 8000 60  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/on-semiconductor/NCP1072STAT3G/NCP1072STAT3GOSCT-ND/3597819" H 1050 1200 60  0001 C CNN "Supplier"
	1    11600 8000
	1    0    0    -1  
$EndComp
$Comp
L R R21
U 1 1 59FB5B61
P 13800 8050
F 0 "R21" V 13880 8050 50  0000 C CNN
F 1 "R" V 13800 8050 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 13730 8050 50  0001 C CNN
F 3 "" H 13800 8050 50  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-0710KL/311-10KGRCT-ND/729647" H 1050 1200 60  0001 C CNN "Supplier"
	1    13800 8050
	-1   0    0    1   
$EndComp
$Comp
L R R27
U 1 1 59FB6774
P 14100 8600
F 0 "R27" V 14180 8600 50  0000 C CNN
F 1 "R" V 14100 8600 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" V 14030 8600 50  0001 C CNN
F 3 "" H 14100 8600 50  0001 C CNN
F 4 "https://www.digikey.com/product-detail/en/yageo/RC0603JR-0710KL/311-10KGRCT-ND/729647" H 1050 1200 60  0001 C CNN "Supplier"
	1    14100 8600
	0    -1   -1   0   
$EndComp
$Comp
L CP C23
U 1 1 59FB9019
P 15150 7300
F 0 "C23" H 15268 7346 50  0000 L CNN
F 1 "470uF" H 15268 7255 50  0000 L CNN
F 2 "Capacitors_THT:CP_Radial_D8.0mm_P3.50mm" H 15188 7150 50  0001 C CNN
F 3 "" H 15150 7300 50  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/wurth-electronics-inc/860020273011/732-8913-1-ND/5728856" H 1050 1200 60  0001 C CNN "Supplier"
	1    15150 7300
	1    0    0    -1  
$EndComp
$Comp
L C C32
U 1 1 59FB95CD
P 15500 7300
F 0 "C32" H 15615 7346 50  0000 L CNN
F 1 "47u" H 15615 7255 50  0000 L CNN
F 2 "Capacitors_SMD:C_1206" H 15538 7150 50  0001 C CNN
F 3 "" H 15500 7300 50  0000 C CNN
F 4 "https://www.digikey.ch/product-detail/de/murata-electronics-north-america/GRM32ER71A476KE15L/490-5312-1-ND/2039091" H 1050 1200 60  0001 C CNN "Supplier"
	1    15500 7300
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR043
U 1 1 59FC8547
P 13800 2400
F 0 "#PWR043" H 13800 2150 60  0001 C CNN
F 1 "GND" V 13800 2200 60  0000 C CNN
F 2 "" H 13800 2400 60  0000 C CNN
F 3 "" H 13800 2400 60  0000 C CNN
	1    13800 2400
	0    1    1    0   
$EndComp
$Comp
L +3V3 #PWR044
U 1 1 59FCBFF9
P 13800 1950
F 0 "#PWR044" H 13800 1800 50  0001 C CNN
F 1 "+3V3" H 13815 2123 50  0000 C CNN
F 2 "" H 13800 1950 50  0000 C CNN
F 3 "" H 13800 1950 50  0000 C CNN
	1    13800 1950
	0    -1   -1   0   
$EndComp
$Comp
L R R18
U 1 1 59D31E17
P 5950 2150
F 0 "R18" V 5850 2100 50  0000 L CNN
F 1 "1k" V 5950 2100 50  0000 L CNN
F 2 "Resistors_SMD:R_0603" V 5880 2150 50  0001 C CNN
F 3 "" H 5950 2150 50  0000 C CNN
F 4 "https://www.digikey.ch/product-detail/de/yageo/RC0603JR-071KL/311-1.0KGRCT-ND/729624" H 1050 1200 60  0001 C CNN "Supplier"
	1    5950 2150
	-1   0    0    1   
$EndComp
$Comp
L +5V #PWR011
U 1 1 59FCE76C
P 10150 1650
F 0 "#PWR011" H 10150 1500 50  0001 C CNN
F 1 "+5V" H 10150 1790 50  0000 C CNN
F 2 "" H 10150 1650 50  0001 C CNN
F 3 "" H 10150 1650 50  0001 C CNN
	1    10150 1650
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR013
U 1 1 59FCF2DA
P 5950 1050
F 0 "#PWR013" H 5950 900 50  0001 C CNN
F 1 "+5V" H 5950 1190 50  0000 C CNN
F 2 "" H 5950 1050 50  0001 C CNN
F 3 "" H 5950 1050 50  0001 C CNN
	1    5950 1050
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR045
U 1 1 59FD28CD
P 3200 1700
F 0 "#PWR045" H 3200 1550 50  0001 C CNN
F 1 "+5V" H 3200 1840 50  0000 C CNN
F 2 "" H 3200 1700 50  0001 C CNN
F 3 "" H 3200 1700 50  0001 C CNN
	1    3200 1700
	1    0    0    -1  
$EndComp
$Comp
L Q_PMOS_GSD Q1
U 1 1 59FD539F
P 3300 2100
F 0 "Q1" H 3506 2054 50  0000 L CNN
F 1 "BSS84" H 3506 2145 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-23" H 3500 2200 50  0001 C CNN
F 3 "" H 3300 2100 50  0001 C CNN
F 4 "https://www.digikey.ch/product-detail/de/diodes-incorporated/BSS84-7-F/BSS84-FDICT-ND/717844" H 1050 1200 60  0001 C CNN "Supplier"
	1    3300 2100
	-1   0    0    1   
$EndComp
$Comp
L Loop_Antenna A2
U 1 1 59FF7F7B
P 1350 9500
F 0 "A2" H 1150 9850 60  0000 L CNN
F 1 "Loop_Antenna" H 1550 9350 60  0001 L CNN
F 2 "antennas:NFC_TAG_40x50" H 850 9750 60  0001 C CNN
F 3 "" H 850 9750 60  0001 C CNN
	1    1350 9500
	1    0    0    -1  
$EndComp
Text Notes 13650 6800 0    60   ~ 0
27V\n1A\n
$Comp
L Conn_01x05_Female J2
U 1 1 5A023881
P 8400 2350
F 0 "J2" H 8427 2376 50  0000 L CNN
F 1 "Conn_01x05_Female" H 8427 2285 50  0000 L CNN
F 2 "Socket_Strips:Socket_Strip_Straight_1x05_Pitch2.54mm" H 8400 2350 50  0001 C CNN
F 3 "~" H 8400 2350 50  0001 C CNN
	1    8400 2350
	1    0    0    -1  
$EndComp
$Comp
L Conn_01x05_Male J4
U 1 1 5A023C45
P 8850 2350
F 0 "J4" H 8956 2728 50  0000 C CNN
F 1 "Conn_01x05_Male" H 8956 2637 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_1x05_Pitch2.54mm" H 8850 2350 50  0001 C CNN
F 3 "~" H 8850 2350 50  0001 C CNN
F 4 "https://www.digikey.ch/product-detail/en/te-connectivity-amp-connectors/4-146468-0/A108562-ND/4030896" H 8850 2350 60  0001 C CNN "Supplier"
	1    8850 2350
	1    0    0    -1  
$EndComp
$Comp
L Conn_01x05_Female J1
U 1 1 5A0246EA
P 8400 1250
F 0 "J1" H 8427 1276 50  0000 L CNN
F 1 "Conn_01x05_Female" H 8427 1185 50  0000 L CNN
F 2 "Socket_Strips:Socket_Strip_Straight_1x05_Pitch2.54mm" H 8400 1250 50  0001 C CNN
F 3 "~" H 8400 1250 50  0001 C CNN
F 4 "https://www.digikey.ch/product-detail/en/sullins-connector-solutions/PPTC061LFBN-RC/S7004-ND/810145" H 8400 1250 60  0001 C CNN "Supplier"
	1    8400 1250
	1    0    0    -1  
$EndComp
$Comp
L Conn_01x05_Male J3
U 1 1 5A0247CF
P 8850 1250
F 0 "J3" H 8956 1628 50  0000 C CNN
F 1 "Conn_01x05_Male" H 8956 1537 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_1x05_Pitch2.54mm" H 8850 1250 50  0001 C CNN
F 3 "~" H 8850 1250 50  0001 C CNN
	1    8850 1250
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR046
U 1 1 5A025E19
P 8100 1000
F 0 "#PWR046" H 8100 850 50  0001 C CNN
F 1 "+5V" H 8100 1140 50  0000 C CNN
F 2 "" H 8100 1000 50  0001 C CNN
F 3 "" H 8100 1000 50  0001 C CNN
	1    8100 1000
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR047
U 1 1 5A025EF2
P 9150 1000
F 0 "#PWR047" H 9150 850 50  0001 C CNN
F 1 "+5V" H 9150 1140 50  0000 C CNN
F 2 "" H 9150 1000 50  0001 C CNN
F 3 "" H 9150 1000 50  0001 C CNN
	1    9150 1000
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR048
U 1 1 5A025FCB
P 9200 2100
F 0 "#PWR048" H 9200 1950 50  0001 C CNN
F 1 "+5V" H 9200 2240 50  0000 C CNN
F 2 "" H 9200 2100 50  0001 C CNN
F 3 "" H 9200 2100 50  0001 C CNN
	1    9200 2100
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR049
U 1 1 5A0260A4
P 8100 2100
F 0 "#PWR049" H 8100 1950 50  0001 C CNN
F 1 "+5V" H 8100 2240 50  0000 C CNN
F 2 "" H 8100 2100 50  0001 C CNN
F 3 "" H 8100 2100 50  0001 C CNN
	1    8100 2100
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR050
U 1 1 5A02617D
P 9200 2600
F 0 "#PWR050" H 9200 2350 50  0001 C CNN
F 1 "GND" H 9205 2427 50  0000 C CNN
F 2 "" H 9200 2600 50  0000 C CNN
F 3 "" H 9200 2600 50  0000 C CNN
	1    9200 2600
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR051
U 1 1 5A026256
P 8100 2600
F 0 "#PWR051" H 8100 2350 50  0001 C CNN
F 1 "GND" H 8105 2427 50  0000 C CNN
F 2 "" H 8100 2600 50  0000 C CNN
F 3 "" H 8100 2600 50  0000 C CNN
	1    8100 2600
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR052
U 1 1 5A026AB6
P 9150 1500
F 0 "#PWR052" H 9150 1250 50  0001 C CNN
F 1 "GND" H 9155 1327 50  0000 C CNN
F 2 "" H 9150 1500 50  0000 C CNN
F 3 "" H 9150 1500 50  0000 C CNN
	1    9150 1500
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR053
U 1 1 5A026B8F
P 8100 1500
F 0 "#PWR053" H 8100 1250 50  0001 C CNN
F 1 "GND" H 8105 1327 50  0000 C CNN
F 2 "" H 8100 1500 50  0000 C CNN
F 3 "" H 8100 1500 50  0000 C CNN
	1    8100 1500
	1    0    0    -1  
$EndComp
Text Label 8200 1350 2    60   ~ 0
I1P_
Text Label 8200 1250 2    60   ~ 0
I1N_
Text Label 9050 1350 0    60   ~ 0
I1P
Text Label 9050 1250 0    60   ~ 0
I1N
Text Label 8200 2250 2    60   ~ 0
R_ON_
Text Label 8200 1150 2    60   ~ 0
T_ON_
Text Label 8200 2350 2    60   ~ 0
VP_
Text Label 9050 2250 0    60   ~ 0
R_ON
Text Label 9050 1150 0    60   ~ 0
T_ON
Text Label 9050 2350 0    60   ~ 0
VP
$Comp
L 2PScrewConn X1
U 1 1 5A057BD3
P 2600 1000
F 0 "X1" H 2372 952 60  0000 R CNN
F 1 "Neutral In/Out" H 2372 1058 60  0000 R CNN
F 2 "on-shore:ED365-2" H 2600 1000 60  0001 C CNN
F 3 "" H 2600 1000 60  0001 C CNN
F 4 "https://www.digikey.ch/product-detail/de/on-shore-technology-inc/ED365-2/ED2354-ND/299652" H 2600 1000 60  0001 C CNN "Supplier"
	1    2600 1000
	-1   0    0    1   
$EndComp
Text Label 2400 900  2    60   ~ 0
N
Text Label 2400 1100 2    60   ~ 0
I_IN
$Comp
L C C37
U 1 1 5A062FD0
P 1750 9700
F 0 "C37" H 1775 9800 50  0000 L CNN
F 1 "TBD" V 1600 9500 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 1788 9550 30  0001 C CNN
F 3 "" H 1750 9700 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM1555C1HR80BA01D/490-6269-1-ND/3845466" H 550 1200 60  0001 C CNN "Supplier"
	1    1750 9700
	1    0    0    -1  
$EndComp
$Comp
L L_Small L6
U 1 1 5A06409D
P 2000 9550
F 0 "L6" V 2050 9650 50  0000 C CNN
F 1 "TBD" V 2150 9550 50  0000 C CNN
F 2 "Capacitors_SMD:C_0603" H 2000 9550 50  0001 C CNN
F 3 "" H 2000 9550 50  0001 C CNN
	1    2000 9550
	0    1    1    0   
$EndComp
Wire Notes Line
	600  600  600  3150
Wire Notes Line
	600  3150 4750 3150
Wire Notes Line
	4750 600  600  600 
Wire Notes Line
	4850 3150 7850 3150
Wire Notes Line
	7850 3150 7850 600 
Wire Notes Line
	7850 600  4850 600 
Wire Wire Line
	850  4150 2250 4150
Wire Wire Line
	1700 4150 1700 4000
Connection ~ 1700 4150
Wire Wire Line
	1600 4450 2250 4450
Wire Wire Line
	2250 4250 2200 4250
Connection ~ 1150 4150
Wire Wire Line
	850  4450 1300 4450
Wire Wire Line
	1150 4450 1150 4550
Wire Wire Line
	2250 4550 2100 4550
Connection ~ 1700 4450
Connection ~ 1150 4450
Wire Wire Line
	850  4800 2250 4800
Connection ~ 1150 4800
Wire Wire Line
	850  5100 1450 5100
Wire Wire Line
	1450 4900 2250 4900
Connection ~ 1150 5100
Wire Wire Line
	1550 5100 2250 5100
Wire Wire Line
	1450 5100 1450 4900
Wire Wire Line
	5700 4150 6350 4150
Wire Wire Line
	6350 4550 5700 4550
Wire Wire Line
	7150 3950 7150 4200
Wire Wire Line
	7150 4500 7150 4750
Wire Wire Line
	7150 4750 7300 4750
Connection ~ 7150 4550
Wire Wire Line
	7150 3950 7300 3950
Connection ~ 7150 4150
Connection ~ 7750 4350
Wire Wire Line
	7750 4050 7750 3950
Wire Wire Line
	7600 3950 8000 3950
Wire Wire Line
	7600 4750 8000 4750
Wire Wire Line
	7750 4750 7750 4650
Wire Wire Line
	7750 4350 8150 4350
Wire Wire Line
	2250 5750 2000 5750
Wire Wire Line
	2000 5650 2250 5650
Connection ~ 7750 3950
Connection ~ 7750 4750
Wire Wire Line
	2250 5400 2000 5400
Wire Wire Line
	2000 5300 2250 5300
Wire Wire Line
	6550 5800 6350 5800
Wire Wire Line
	6350 5800 6350 5700
Wire Wire Line
	6350 5400 6350 5300
Wire Wire Line
	5700 6100 7350 6100
Connection ~ 6350 6100
Wire Wire Line
	6350 5000 5700 5000
Wire Wire Line
	7050 5800 7350 5800
Connection ~ 6550 6100
Connection ~ 7050 6100
Wire Wire Line
	2250 5950 2000 5950
Wire Wire Line
	2000 6050 2250 6050
Wire Notes Line
	600  3250 600  6300
Wire Notes Line
	600  6300 8850 6300
Wire Notes Line
	8850 6300 8850 3250
Wire Notes Line
	8850 3250 600  3250
Wire Notes Line
	5750 4800 5550 4800
Wire Notes Line
	5550 4800 5550 6200
Wire Notes Line
	5550 6200 5750 6200
Wire Notes Line
	5750 4650 5550 4650
Wire Notes Line
	5550 4650 5550 3950
Wire Notes Line
	5550 3950 5750 3950
Connection ~ 6350 5800
Wire Wire Line
	4200 4500 4500 4500
Wire Wire Line
	4400 4500 4400 4600
Wire Wire Line
	4400 4800 4400 4900
Wire Wire Line
	4200 4900 4500 4900
Wire Wire Line
	4800 4500 4800 4900
Wire Wire Line
	4950 4700 4800 4700
Connection ~ 4800 4700
Wire Wire Line
	4200 4650 4200 4500
Connection ~ 4400 4500
Wire Wire Line
	4200 4750 4200 4900
Connection ~ 4400 4900
Wire Wire Line
	3450 4150 3750 4150
Wire Wire Line
	3750 4250 3450 4250
Wire Wire Line
	3450 4350 3750 4350
Wire Wire Line
	3750 4450 3450 4450
Wire Wire Line
	3450 5950 4050 5950
Wire Wire Line
	4050 5950 4050 6050
Wire Wire Line
	4050 6050 3450 6050
Wire Wire Line
	4050 5750 3450 5750
Wire Wire Line
	3450 5650 4050 5650
Wire Wire Line
	4050 5450 3450 5450
Wire Wire Line
	3450 5350 4050 5350
Wire Wire Line
	4050 5250 3450 5250
Wire Wire Line
	4050 4950 3450 4950
Wire Wire Line
	3450 5050 4050 5050
Wire Wire Line
	3450 4650 4200 4650
Wire Wire Line
	4200 4750 3450 4750
Wire Notes Line
	12950 600  15950 600 
Wire Notes Line
	15950 600  15950 3150
Wire Notes Line
	15950 3150 12950 3150
Wire Notes Line
	12950 3150 12950 600 
Wire Wire Line
	13800 1950 14050 1950
Wire Wire Line
	14100 2400 13800 2400
Wire Wire Line
	15250 2400 14700 2400
Wire Wire Line
	14650 1950 15250 1950
Wire Wire Line
	14250 1100 14450 1100
Wire Wire Line
	14250 1500 14450 1500
Wire Wire Line
	13800 1100 13950 1100
Wire Wire Line
	13950 1500 13800 1500
Wire Wire Line
	14750 1100 15250 1100
Wire Wire Line
	14750 1500 15250 1500
Wire Notes Line
	4850 600  4850 3150
Wire Notes Line
	4750 3150 4750 600 
Wire Wire Line
	1250 2800 3200 2800
Connection ~ 2100 2800
Connection ~ 3200 2800
Wire Wire Line
	2100 2800 2100 2450
Wire Wire Line
	2900 1650 2100 1650
Wire Wire Line
	2100 1650 2100 1800
Wire Wire Line
	6950 1250 7300 1250
Wire Wire Line
	6950 1550 7300 1550
Wire Wire Line
	3200 2800 3200 2750
Wire Wire Line
	3850 2100 3500 2100
Wire Wire Line
	2700 2400 2600 2400
Wire Notes Line
	7950 6400 7950 9500
Wire Notes Line
	7950 9500 15950 9500
Wire Notes Line
	15950 9500 15950 6400
Wire Notes Line
	15950 6400 7950 6400
Wire Wire Line
	9750 6600 9750 7000
Wire Wire Line
	9750 7300 9750 9300
Wire Wire Line
	9600 6600 12500 6600
Connection ~ 9750 8100
Wire Wire Line
	10100 9300 10100 9050
Connection ~ 10100 9300
Wire Wire Line
	11100 9300 11100 9050
Wire Wire Line
	11100 8000 11100 8750
Wire Wire Line
	8500 8100 9750 8100
Wire Wire Line
	11200 7900 10100 7900
Wire Wire Line
	10100 7850 10100 8750
Wire Wire Line
	10800 7500 10100 7500
Connection ~ 10100 7900
Wire Wire Line
	15500 7100 15500 7150
Wire Wire Line
	14900 6650 14900 7900
Connection ~ 14900 7100
Wire Wire Line
	14300 7100 14300 7900
Connection ~ 14300 7100
Wire Wire Line
	14900 8200 14900 8800
Wire Wire Line
	14900 9300 14900 9100
Connection ~ 11100 9300
Wire Wire Line
	11400 6600 11400 6800
Wire Wire Line
	11400 7100 11400 7300
Wire Wire Line
	11400 7300 12000 7300
Connection ~ 11800 7300
Connection ~ 11400 6600
Wire Wire Line
	12300 7300 12700 7300
Connection ~ 12400 7300
Wire Wire Line
	11100 7500 12700 7500
Wire Wire Line
	12700 8400 11100 8400
Connection ~ 11100 8400
Wire Wire Line
	12700 8600 12500 8600
Wire Wire Line
	12500 9300 12500 7700
Connection ~ 12500 9300
Wire Wire Line
	13300 8600 13550 8600
Wire Wire Line
	14300 8200 14300 8400
Wire Wire Line
	14300 8400 13300 8400
Wire Wire Line
	14250 8600 14900 8600
Connection ~ 14900 8600
Wire Wire Line
	15500 9300 15500 7450
Connection ~ 14900 9300
Wire Wire Line
	12500 6600 12500 6900
Connection ~ 11800 6600
Wire Wire Line
	12500 6900 12700 6900
Wire Wire Line
	12500 7700 12700 7700
Connection ~ 12500 8600
Wire Wire Line
	13600 7100 13400 7100
Wire Wire Line
	13400 7500 15500 7500
Connection ~ 15500 7500
Connection ~ 13450 9300
Wire Wire Line
	13450 8300 13450 8650
Connection ~ 13450 8600
Wire Wire Line
	13450 9300 13450 9200
Wire Wire Line
	13700 8950 14400 8950
Wire Wire Line
	14400 8950 14400 8600
Connection ~ 14400 8600
Wire Wire Line
	13900 7100 15500 7100
Wire Wire Line
	9750 9300 15500 9300
Wire Wire Line
	9600 7550 9600 6600
Connection ~ 9750 6600
Wire Wire Line
	8500 7550 8500 8100
Wire Wire Line
	9000 7100 9000 7250
Wire Wire Line
	9000 8000 9000 7850
Wire Wire Line
	8100 8000 9000 8000
Wire Wire Line
	8100 7100 9000 7100
Wire Wire Line
	8500 7550 8700 7550
Wire Wire Line
	9600 7550 9300 7550
Connection ~ 11800 6950
Wire Wire Line
	11800 7250 11800 7300
Wire Wire Line
	11800 6650 11800 6600
Wire Notes Line
	7950 3150 7950 600 
Wire Notes Line
	12850 3150 7950 3150
Wire Notes Line
	12850 600  12850 3150
Wire Notes Line
	7950 600  12850 600 
Wire Wire Line
	12050 1800 12350 1800
Connection ~ 11550 1800
Wire Wire Line
	10850 2200 10850 2300
Connection ~ 10850 2200
Wire Wire Line
	11550 2200 11550 2100
Wire Wire Line
	11400 1800 11850 1800
Connection ~ 9900 2200
Connection ~ 9900 1800
Wire Wire Line
	9900 2100 9900 2200
Wire Wire Line
	11100 4650 11100 4700
Connection ~ 11100 4200
Wire Wire Line
	11100 4200 11100 4350
Wire Wire Line
	14550 4850 15050 4850
Wire Wire Line
	15050 4750 14550 4750
Wire Wire Line
	14550 4650 15050 4650
Wire Wire Line
	14550 4550 15050 4550
Wire Wire Line
	14550 4450 15050 4450
Wire Notes Line
	15950 3250 13800 3250
Wire Notes Line
	15950 6300 15950 3250
Wire Notes Line
	13800 6300 15950 6300
Wire Notes Line
	13800 3250 13800 6300
Wire Notes Line
	8950 6300 8950 3250
Wire Notes Line
	13700 6300 8950 6300
Wire Notes Line
	13700 3250 13700 6300
Wire Notes Line
	8950 3250 13700 3250
Wire Wire Line
	6250 9850 5700 9850
Wire Wire Line
	3500 8850 2700 8850
Wire Wire Line
	2700 8950 3500 8950
Wire Wire Line
	3500 9050 2700 9050
Wire Wire Line
	6100 8150 5700 8150
Wire Wire Line
	6600 9650 6550 9650
Wire Wire Line
	4250 10700 4050 10700
Wire Wire Line
	5150 10500 4950 10500
Wire Wire Line
	4950 10700 5450 10700
Wire Wire Line
	5700 9750 6250 9750
Wire Wire Line
	6600 9450 6550 9450
Wire Wire Line
	5700 8950 6250 8950
Wire Wire Line
	6250 9450 5700 9450
Wire Wire Line
	6250 9050 5700 9050
Wire Wire Line
	6100 7750 5700 7750
Wire Wire Line
	2700 9250 3500 9250
Wire Wire Line
	3500 9150 2700 9150
Wire Wire Line
	3500 8250 3350 8250
Wire Wire Line
	3500 8350 3350 8350
Wire Wire Line
	3750 10500 4250 10500
Wire Wire Line
	5700 9650 6250 9650
Wire Wire Line
	6100 7850 5700 7850
Wire Wire Line
	5700 7950 6100 7950
Wire Wire Line
	6100 8250 5700 8250
Wire Wire Line
	6250 9350 5700 9350
Wire Wire Line
	6250 9150 5700 9150
Wire Wire Line
	5700 8750 6250 8750
Wire Wire Line
	5700 8850 6250 8850
Wire Wire Line
	5700 7250 6900 7250
Wire Wire Line
	7100 7250 7650 7250
Wire Notes Line
	7850 6400 600  6400
Wire Notes Line
	7850 11100 7850 6400
Wire Notes Line
	600  11100 7850 11100
Wire Notes Line
	600  6400 600  11100
Wire Notes Line
	7200 7950 6500 7950
Wire Notes Line
	7200 7050 7200 7950
Wire Notes Line
	6500 7050 7200 7050
Wire Notes Line
	6500 7950 6500 7050
Connection ~ 6750 7250
Connection ~ 6750 7650
Wire Wire Line
	6575 7350 5700 7350
Wire Wire Line
	6575 7650 6575 7350
Wire Wire Line
	6750 7650 6575 7650
Wire Wire Line
	6750 7600 6750 7700
Wire Wire Line
	6750 7300 6750 7250
Wire Wire Line
	2850 8200 2900 8200
Connection ~ 3450 7450
Connection ~ 3450 7350
Connection ~ 3450 7250
Wire Wire Line
	3450 7150 3450 7450
Wire Wire Line
	2650 8700 2650 8650
Wire Wire Line
	2650 8650 3500 8650
Wire Wire Line
	2650 8500 2650 8550
Connection ~ 2250 9550
Wire Wire Line
	3200 8200 3300 8200
Connection ~ 2500 8700
Wire Wire Line
	2400 8700 2650 8700
Connection ~ 2500 8500
Wire Wire Line
	2650 8550 3500 8550
Wire Wire Line
	2400 8500 2650 8500
Wire Wire Line
	2600 8050 3500 8050
Wire Wire Line
	2600 7950 3500 7950
Wire Wire Line
	2350 8050 2400 8050
Wire Wire Line
	2350 7950 2350 8050
Wire Wire Line
	2400 7950 2350 7950
Wire Wire Line
	3350 7850 3500 7850
Wire Wire Line
	3500 7750 3350 7750
Wire Wire Line
	3350 7650 3500 7650
Wire Wire Line
	3350 7450 3500 7450
Wire Wire Line
	3350 7350 3500 7350
Wire Wire Line
	3350 7250 3500 7250
Wire Wire Line
	3050 7250 3000 7250
Wire Wire Line
	3000 7350 3050 7350
Wire Wire Line
	3050 7450 3000 7450
Wire Wire Line
	3000 7650 3050 7650
Wire Wire Line
	3050 7750 3000 7750
Wire Wire Line
	3000 7850 3050 7850
Wire Wire Line
	3400 9850 3400 10100
Connection ~ 3300 7950
Wire Wire Line
	3300 8200 3300 7950
Connection ~ 3400 9950
Wire Wire Line
	3400 9950 3500 9950
Wire Wire Line
	3500 9850 3400 9850
Connection ~ 9750 7400
Wire Wire Line
	11100 8000 11200 8000
Wire Wire Line
	12400 7300 12400 7650
Wire Wire Line
	12400 7650 10950 7650
Wire Wire Line
	10950 7650 10950 8100
Wire Wire Line
	10950 8100 11200 8100
Wire Wire Line
	12000 8000 12050 8000
Wire Wire Line
	12050 8000 12050 8050
Wire Wire Line
	10100 7500 10100 7550
Wire Wire Line
	13800 8200 13800 8300
Wire Wire Line
	13800 8300 13450 8300
Wire Wire Line
	13800 7900 13800 7650
Wire Wire Line
	13800 7650 14100 7650
Wire Wire Line
	14100 7650 14100 7100
Connection ~ 14100 7100
Wire Wire Line
	13850 8600 13950 8600
Wire Wire Line
	15150 7150 15150 7100
Connection ~ 15150 7100
Wire Wire Line
	15150 7450 15150 7500
Connection ~ 15150 7500
Wire Wire Line
	9900 1800 10300 1800
Wire Wire Line
	9900 2200 11550 2200
Wire Wire Line
	5950 1800 5950 2000
Wire Wire Line
	6250 2000 6250 2400
Wire Wire Line
	5950 2300 5950 2350
Wire Wire Line
	5950 2350 6250 2350
Connection ~ 6250 2350
Wire Wire Line
	6250 1500 6250 1600
Wire Wire Line
	5950 1900 5600 1900
Connection ~ 5950 1900
Wire Wire Line
	10150 1650 10150 1800
Connection ~ 10150 1800
Wire Wire Line
	5650 1200 5650 1150
Wire Wire Line
	5650 1150 6250 1150
Wire Wire Line
	6250 1150 6250 1300
Connection ~ 5950 1150
Wire Wire Line
	5950 1050 5950 1150
Wire Wire Line
	6250 1550 5650 1550
Wire Wire Line
	5650 1550 5650 1500
Connection ~ 6250 1550
Wire Wire Line
	3200 1700 3200 1900
Wire Wire Line
	3200 2300 3200 2450
Connection ~ 3200 2400
Wire Wire Line
	3000 2400 3200 2400
Wire Wire Line
	8200 1450 8100 1450
Wire Wire Line
	8100 1450 8100 1500
Wire Wire Line
	9150 1000 9150 1050
Wire Wire Line
	9150 1050 9050 1050
Wire Wire Line
	8100 1000 8100 1050
Wire Wire Line
	8100 1050 8200 1050
Wire Wire Line
	9150 1500 9150 1450
Wire Wire Line
	9150 1450 9050 1450
Wire Wire Line
	9200 2100 9200 2150
Wire Wire Line
	9200 2150 9050 2150
Wire Wire Line
	9050 2550 9200 2550
Wire Wire Line
	9200 2550 9200 2600
Wire Wire Line
	8100 2600 8100 2550
Wire Wire Line
	8100 2550 8200 2550
Wire Wire Line
	8100 2100 8100 2150
Wire Wire Line
	8100 2150 8200 2150
Connection ~ 1750 9550
Wire Wire Line
	1700 9550 1900 9550
Wire Wire Line
	2100 9550 3500 9550
$Comp
L GND #PWR054
U 1 1 5A067848
P 1750 9850
F 0 "#PWR054" H 1750 9600 60  0001 C CNN
F 1 "GND" H 1750 9700 60  0000 C CNN
F 2 "" H 1750 9850 60  0000 C CNN
F 3 "" H 1750 9850 60  0000 C CNN
	1    1750 9850
	1    0    0    -1  
$EndComp
$Comp
L L_Small L5
U 1 1 5A07795B
P 10800 4200
F 0 "L5" V 10900 4200 50  0000 C CNN
F 1 "TBD" V 10700 4200 50  0000 C CNN
F 2 "Capacitors_SMD:C_0603" H 10800 4200 50  0001 C CNN
F 3 "" H 10800 4200 50  0001 C CNN
	1    10800 4200
	0    -1   -1   0   
$EndComp
$Comp
L C C8
U 1 1 5A077AC6
P 10450 4500
F 0 "C8" H 10475 4600 50  0000 L CNN
F 1 "TBD" H 10500 4400 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 10488 4350 30  0001 C CNN
F 3 "" H 10450 4500 60  0000 C CNN
F 4 "https://www.digikey.com/product-detail/en/murata-electronics-north-america/GRM1555C1HR80BA01D/490-6269-1-ND/3845466" H 9250 -4000 60  0001 C CNN "Supplier"
	1    10450 4500
	1    0    0    -1  
$EndComp
Wire Wire Line
	10900 4200 12150 4200
Wire Wire Line
	10050 4200 10700 4200
Wire Wire Line
	10450 4200 10450 4350
Connection ~ 10450 4200
$Comp
L GND #PWR028
U 1 1 5A0793E0
P 10450 4700
F 0 "#PWR028" H 10450 4450 60  0001 C CNN
F 1 "GND" H 10450 4550 60  0000 C CNN
F 2 "" H 10450 4700 60  0000 C CNN
F 3 "" H 10450 4700 60  0000 C CNN
	1    10450 4700
	1    0    0    -1  
$EndComp
Wire Wire Line
	10450 4650 10450 4700
Wire Wire Line
	3500 9450 1700 9450
$Comp
L POSITION_TEST_CIRCLE POSITION_TEST_CIRCLE_00
U 1 1 55030D28
P 0 0
	1    0    0
	0    1    1    0   
$EndComp
$EndSCHEMATC
