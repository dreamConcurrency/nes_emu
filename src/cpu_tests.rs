use super::cpu:: {CPU, Mem };

/* TAX */
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

/* TXA */
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

/* TAY */
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

/* TYA */
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

/* DEY */
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

/* INY */
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

/* DEX */
#[test]
fn test_dex() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;

    cpu.load_and_run(vec![0xca, 0x00]);
    assert_eq!(cpu.register_x, 0);
}

#[test]
fn test_dex_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;

    cpu.load_and_run(vec![0xca, 0x00]);
    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_dex_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 129;

    cpu.load_and_run(vec![0xca, 0x00]);
    assert_eq!(cpu.register_x, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

/* INX */ 
#[test]
fn test_inx() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;

    cpu.load_and_run(vec![0xe8, 0x00]);
    assert_eq!(cpu.register_x, 2);
}

#[test]
fn test_inx_zero_flag_overflow() {
    let mut cpu = CPU::new();
    cpu.register_x = 255;

    cpu.load_and_run(vec![0xe8, 0x00]);
    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_inx_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 127;

    cpu.load_and_run(vec![0xe8, 0x00]);
    assert_eq!(cpu.register_x, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

/* STA */ 
#[test]
fn test_sta_0x85() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;

    cpu.load_and_run(vec![0x85, 0x50, 0x00]);
    assert_eq!(cpu.mem_read(0x0050), 10);
}

#[test]
fn test_sta_0x95() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;
    cpu.register_x = 5;

    cpu.load_and_run(vec![0x95, 0x50, 0x00]);
    assert_eq!(cpu.mem_read(0x0055), 10);
}

#[test]
fn test_sta_0x8d() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;

    cpu.load_and_run(vec![0x8d, 0x50, 0x00, 0x00]);
    assert_eq!(cpu.mem_read(0x0050), 10);
}

#[test]
fn test_sta_0x9d() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;
    cpu.register_x = 5;

    cpu.load_and_run(vec![0x9d, 0x40, 0x00, 0x00]);
    assert_eq!(cpu.mem_read(0x0045), 10);
}

#[test]
fn test_sta_0x99() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;
    cpu.register_y = 7;

    cpu.load_and_run(vec![0x99, 0x40, 0x00, 0x00]);
    assert_eq!(cpu.mem_read(0x0047), 10);
}

#[test]
fn test_sta_0x81() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;
    cpu.register_x = 5;

    cpu.mem_write(0x25, 0x00);
    cpu.mem_write(0x26, 0x01);

    cpu.load_and_run(vec![0x81, 0x20, 0x00]);
    assert_eq!(cpu.mem_read(0x0100), 10);
}

#[test]
fn test_sta_0x91() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;
    cpu.register_y = 5;

    cpu.mem_write(0x20, 0x00);
    cpu.mem_write(0x21, 0x01);

    cpu.load_and_run(vec![0x91, 0x20, 0x00]);
    assert_eq!(cpu.mem_read(0x0105), 10);
}

/* STX */
#[test]
fn test_stx_0x86() {
    let mut cpu = CPU::new();
    cpu.register_x = 10;

    cpu.load_and_run(vec![0x86, 0x50, 0x00]);
    assert_eq!(cpu.mem_read(0x0050), 10);
}

#[test]
fn test_stx_0x96() {
    let mut cpu = CPU::new();
    cpu.register_x = 10;
    cpu.register_y = 5;

    cpu.load_and_run(vec![0x96, 0x50, 0x00]);
    assert_eq!(cpu.mem_read(0x0055), 10);
}

#[test]
fn test_stx_0x8e() {
    let mut cpu = CPU::new();
    cpu.register_x = 10;

    cpu.load_and_run(vec![0x8e, 0x00, 0x10, 0x00]);
    assert_eq!(cpu.mem_read(0x1000), 10);
}

/* STY */ 
#[test]
fn test_sty_0x84() {
   let mut cpu = CPU::new();
   cpu.register_y = 10;

   cpu.load_and_run(vec![0x84, 0x50, 0x00]);
   assert_eq!(cpu.mem_read(0x0050), 10);
}

#[test]
fn test_sty_0x94() {
    let mut cpu = CPU::new();
    cpu.register_y = 10;
    cpu.register_x = 7;

    cpu.load_and_run(vec![0x94, 0x43, 0x00]);
    assert_eq!(cpu.mem_read(0x004a), 10);
}

#[test]
fn test_sty_0x8c() {
    let mut cpu = CPU::new();
    cpu.register_y = 10;

    cpu.load_and_run(vec![0x8c, 0x00, 0x01, 0x00]);
    assert_eq!(cpu.mem_read(0x0100), 10);
}

/* LDA */
#[test]
fn test_lda_0xa9() {
    let mut cpu = CPU::new();
    
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 5);
}

#[test]
fn test_lda_0xa9_zero_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_lda_0xa9_negative_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_lda_0xa5() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x05, 5);
    
    cpu.load_and_run(vec![0xa5, 0x05, 0x00, 0x00]);
    assert_eq!(cpu.register_a, 5);
}

#[test]
fn test_lda_0xa5_zero_flag() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x05, 0);

    cpu.load_and_run(vec![0xa5, 0x05, 0x00, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_lda_0xa5_negative_flag() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x05, 128);
    
    cpu.load_and_run(vec![0xa5, 0x05, 0x00, 0x00]);
    assert_eq!(cpu.register_a, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_lda_0xb5() {
    let mut cpu = CPU::new();
    cpu.register_x = 5;
    cpu.mem_write(0x55, 42);

    cpu.load_and_run(vec![0xb5, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 42);
}

#[test]
fn test_lda_0xb5_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 5;
    cpu.mem_write(0x55, 0);

    cpu.load_and_run(vec![0xb5, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_lda_0xb5_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 5;
    cpu.mem_write(0x55, 128);

    cpu.load_and_run(vec![0xb5, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_lda_0xad() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x50, 5);
    
    cpu.load_and_run(vec![0xad, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 5);
}

#[test]
fn test_lda_0xad_zero_flag() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x50, 0);
    
    cpu.load_and_run(vec![0xad, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_lda_0xad_negative_flag() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x50, 255);
    
    cpu.load_and_run(vec![0xad, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 255);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_lda_0xbd() {
    let mut cpu = CPU::new();
    cpu.register_x = 5;

    cpu.mem_write(0x55, 10);

    cpu.load_and_run(vec![0xbd, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 10);
}

#[test]
fn test_lda_0xbd_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 5;

    cpu.mem_write(0x55, 0);

    cpu.load_and_run(vec![0xbd, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_lda_0xbd_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 5;

    cpu.mem_write(0x55, 128);

    cpu.load_and_run(vec![0xbd, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_lda_0xb9() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x55, 10);

    cpu.load_and_run(vec![0xb9, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 10);
}

#[test]
fn test_lda_0xb9_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x55, 0);

    cpu.load_and_run(vec![0xbd, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_lda_0xb9_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x55, 128);

    cpu.load_and_run(vec![0xb9, 0x50, 0x00]);
    assert_eq!(cpu.register_a, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_lda_0xa1() {
    let mut cpu = CPU::new();
    cpu.register_x = 5;

    cpu.mem_write(0x25, 0x00);
    cpu.mem_write(0x26, 0x01);

    cpu.mem_write(0x0100, 10);

    cpu.load_and_run(vec![0xa1, 0x20, 0x00]);
    assert_eq!(cpu.register_a, 10);
}

#[test]
fn test_lda_0xa1_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 5;

    cpu.mem_write(0x25, 0x00);
    cpu.mem_write(0x26, 0x01);

    cpu.mem_write(0x0100, 0);

    cpu.load_and_run(vec![0xa1, 0x20, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_lda_0xa1_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 5;

    cpu.mem_write(0x25, 0x00);
    cpu.mem_write(0x26, 0x01);

    cpu.mem_write(0x0100, 128);

    cpu.load_and_run(vec![0xa1, 0x20, 0x00]);
    assert_eq!(cpu.register_a, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_lda_0xb1() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x25, 0x00);
    cpu.mem_write(0x26, 0x01);

    cpu.mem_write(0x0105, 10);

    cpu.load_and_run(vec![0xb1, 0x25, 0x00]);
    assert_eq!(cpu.register_a, 10);
} 

#[test]
fn test_lda_0xb1_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x25, 0x00);
    cpu.mem_write(0x26, 0x01);

    cpu.mem_write(0x0105, 0);

    cpu.load_and_run(vec![0xb1, 0x25, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_lda_0xb1_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x25, 0x00);
    cpu.mem_write(0x26, 0x01);

    cpu.mem_write(0x0105, 128);

    cpu.load_and_run(vec![0xb1, 0x25, 0x00]);
    assert_eq!(cpu.register_a, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

/* LDX */
#[test]
fn test_ldx_0xa2() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa2, 0x05, 0x00]);

    assert_eq!(cpu.register_x, 5);
}

#[test]
fn test_ldx_0xa2_zero_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa2, 0x00, 0x00]);

    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_ldx_0xa2_negative_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa2, 0xff, 0x00]);

    assert_eq!(cpu.register_x, 255);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_ldx_0xa6() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x05, 5);

    cpu.load_and_run(vec![0xa6, 0x05, 0x00, 0x00]);

    assert_eq!(cpu.register_x, 5);
}

#[test]
fn test_ldx_0xa6_zero_flag() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x05, 0);

    cpu.load_and_run(vec![0xa6, 0x05, 0x00, 0x00]);

    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_ldx_0xa6_negative_flag() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x05, 128);

    cpu.load_and_run(vec![0xa6, 0x05, 0x00, 0x00]);

    assert_eq!(cpu.register_x, 128);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_ldx_0xb6() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x45, 0x05);

    cpu.load_and_run(vec![0xb6, 0x40, 0x00]);

    assert_eq!(cpu.register_x, 5);
}

#[test]
fn test_ldx_0xb6_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x45, 0x00);

    cpu.load_and_run(vec![0xb6, 0x40, 0x00]);

    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_ldx_0xb6_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x45, 0xff);

    cpu.load_and_run(vec![0xb6, 0x40, 0x00]);

    assert_eq!(cpu.register_x, 255);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_ldx_0xae() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x0025, 0x05);

    cpu.load_and_run(vec![0xae, 0x25, 0x00]);
    assert_eq!(cpu.register_x, 5);
}

#[test]
fn test_ldx_0xae_zero_flag() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x0025, 0x00);

    cpu.load_and_run(vec![0xae, 0x25, 0x00]);
    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_ldx_0xae_negative_flag() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x0025, 0xff);

    cpu.load_and_run(vec![0xae, 0x25, 0x00]);
    assert_eq!(cpu.register_x, 255);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

#[test]
fn test_ldx_0xbe() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x0045, 0x05);

    cpu.load_and_run(vec![0xbe, 0x40, 0x00]);
    assert_eq!(cpu.register_x, 5);
}

#[test]
fn test_ldx_0xbe_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x0045, 0x00);

    cpu.load_and_run(vec![0xbe, 0x40, 0x00]);
    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 != 0);
}

#[test]
fn test_ldx_0xbe_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.mem_write(0x0045, 0xff);

    cpu.load_and_run(vec![0xbe, 0x40, 0x00]);
    assert_eq!(cpu.register_x, 255);
    assert!(cpu.status.bits() & 0b1000_0000 != 0);
}

/* LDY */
#[test]
fn test_ldy_0xa0() {
   let mut cpu = CPU::new();

   cpu.load_and_run(vec![0xa0, 0x05, 0x00]);

   assert_eq!(cpu.register_y, 5);
}
