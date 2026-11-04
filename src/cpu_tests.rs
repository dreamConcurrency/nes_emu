use super::cpu::CPU;

#[test]
fn test_tax() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;

    cpu.load_and_run(vec![0xaa, 0x00]);
    assert_eq!(cpu.register_x, 10);
    assert!(cpu.status.bits() & 0b0000_0010 == 0); 
}

#[test]
fn test_tax_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0;

    cpu.load_and_run(vec![0xaa, 0x00]);
    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_tax_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 128;

    cpu.load_and_run(vec![0xaa, 0xaa]);
    assert_eq!(cpu.register_x, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_txa() {
    let mut cpu = CPU::new();
    cpu.register_x = 10;
    
    cpu.load_and_run(vec![0x8a, 0x00]);
    assert_eq!(cpu.register_a, 10);
    assert!(cpu.status.bits() & 0b0000_0010 == 0);
}

#[test]
fn test_txa_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 0;

    cpu.load_and_run(vec![0x8a, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_txa_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 128;

    cpu.load_and_run(vec![0x8a, 0x00]);
    assert_eq!(cpu.register_a, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_tay() {
    let mut cpu = CPU::new();
    cpu.register_a = 5;

    cpu.load_and_run(vec![0xa8, 0x00]);
    assert_eq!(cpu.register_y, 5);
}

#[test]
fn test_tay_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0;

    cpu.load_and_run(vec![0xa8, 0x00]);
    assert_eq!(cpu.register_y, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_tay_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 128;

    cpu.load_and_run(vec![0xa8, 0x00]);
    assert_eq!(cpu.register_y, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_tya() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.load_and_run(vec![0x98, 0x00]);
    assert_eq!(cpu.register_a, 5);
}

#[test]
fn test_tya_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 0;

    cpu.load_and_run(vec![0x98, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_tya_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 128;

    cpu.load_and_run(vec![0x98, 0x00]);
    assert_eq!(cpu.register_a, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_dey() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;

    cpu.load_and_run(vec![0x88, 0x00]);
    assert_eq!(cpu.register_y, 0);
}

#[test]
fn test_dey_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;

    cpu.load_and_run(vec![0x88, 0x00]);
    assert_eq!(cpu.register_y, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_dey_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 129;

    cpu.load_and_run(vec![0x88, 0x00]);
    assert_eq!(cpu.register_y, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_iny() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;

    cpu.load_and_run(vec![0xc8, 0x00]);
    assert_eq!(cpu.register_y, 2);
}

#[test]
fn test_iny_zero_flag_overflow() {
    let mut cpu = CPU::new();
    cpu.register_y = 255;

    cpu.load_and_run(vec![0xc8, 0x00]);
    assert_eq!(cpu.register_y, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_iny_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 127;

    cpu.load_and_run(vec![0xc8, 0x00]);
    assert_eq!(cpu.register_y, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}
