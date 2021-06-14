#include <elf.h>
#include <stdio.h>

int main(void) {
  FILE *fp = fopen("./sample_elf", "rb");
  int i, read_cnt;

  char magic_number[4];
  read_cnt = fread(magic_number, sizeof(char), 4, fp);
  printf("ELF Magic Number: ");
  for (i = 0; i < 4; i++) {
    printf("%x ", magic_number[i]);
  }
  printf("\n");

  fseek(fp, 4, SEEK_SET);
  char elf_class;
  if (fread(&elf_class, sizeof(char), 1, fp) != 1) {
    printf("Load failed.\n");
    return 1;
  }
  if (elf_class == ELFCLASS32) {
    printf("ELF Class: 32bit\n");
    Elf32_Ehdr ehdr32;
    if (fread(&ehdr32.e_type, sizeof(ehdr32) - EI_NIDENT, 1, fp) != 1) {
      printf("Load failed.\n");
      return 1;
    }
    printf("ELF Type: %d\n", ehdr32.e_type);
  } else {
    printf("ELF Class: 64bit\n");
    Elf64_Ehdr ehdr64;
    if (fread(&ehdr64.e_type, sizeof(ehdr64) - EI_NIDENT, 1, fp) != 1) {
      printf("Load failed.\n");
      return 1;
    }
    printf("ELF Type: %d\n", ehdr64.e_type);
  }
  
  return 0;
}
