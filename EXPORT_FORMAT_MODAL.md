# Export Format Selection Modal - Systrix

## Overview

Fitur baru yang memungkinkan pengguna memilih format export (CSV, JSON, atau HTML) melalui modal interaktif ketika menekan tombol `e` di TUI.

## Cara Menggunakan

### 1. Membuka Modal Export Format

Tekan tombol `e` di TUI untuk membuka modal pilihan format export.

### 2. Navigasi di Modal

Ada 3 cara untuk memilih format:

#### A. Menggunakan Tombol Angka
- Tekan `1` untuk CSV
- Tekan `2` untuk JSON  
- Tekan `3` untuk HTML

#### B. Menggunakan Arrow Keys
- Tekan `↑` (Up) untuk pindah ke pilihan atas
- Tekan `↓` (Down) untuk pindah ke pilihan bawah

#### C. Menggunakan Enter
- Gunakan arrow keys untuk memilih format
- Tekan `Enter` untuk export dengan format yang dipilih

### 3. Membatalkan Export

Tekan `ESC` untuk menutup modal tanpa melakukan export.

## Format Export yang Tersedia

### 1. CSV (Comma-Separated Values)
- Format tabel sederhana
- Mudah dibuka di Excel/Google Sheets
- Cocok untuk analisis data

### 2. JSON (JavaScript Object Notation)
- Format terstruktur
- Mudah diproses oleh program
- Cocok untuk integrasi API

### 3. HTML (Interactive Web Page)
- Tampilan visual yang indah
- Interaktif dengan sorting dan search
- Cocok untuk laporan dan presentasi

## Tampilan Modal

```
┌─────────────────────────────────────────────┐
│ 📤 Export Data                              │
├─────────────────────────────────────────────┤
│ Select export format:                       │
│                                             │
│ →  [1] CSV  - Comma-separated values       │
│    [2] JSON - JavaScript Object Notation   │
│    [3] HTML - Interactive web page         │
│                                             │
│ ┌──────────────────────────────────────────┐│
│ │  Use [1-3] or [↑↓] to select            ││
│ │  Press [ENTER] to export                ││
│ │  Press [ESC] to cancel                  ││
│ └──────────────────────────────────────────┘│
└─────────────────────────────────────────────┘
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `e` | Buka modal export format |
| `1` | Pilih CSV (di dalam modal) |
| `2` | Pilih JSON (di dalam modal) |
| `3` | Pilih HTML (di dalam modal) |
| `↑` | Navigasi ke atas (di dalam modal) |
| `↓` | Navigasi ke bawah (di dalam modal) |
| `Enter` | Konfirmasi dan export |
| `ESC` | Batalkan dan tutup modal |

## Shortcut Langsung (Tanpa Modal)

Jika ingin export langsung tanpa membuka modal:

- `Ctrl+C` - Export ke CSV langsung
- `Ctrl+H` - Export ke HTML langsung

**Note**: Tombol `e` sekarang membuka modal pilihan, tidak langsung export JSON lagi.

## Implementasi Teknis

### File yang Dimodifikasi

1. **src/tui/ui.rs**
   - Menambahkan `ModalType::ExportFormat`
   - Menambahkan field `export_format_selection: usize`
   - Menambahkan method `show_export_format_modal()`
   - Menambahkan method `export_format_navigate()`
   - Menambahkan method `export_format_select()`
   - Menambahkan method `get_selected_export_format()`
   - Menambahkan method `is_export_format_modal()`

2. **src/tui/event.rs**
   - Mengubah handler tombol `e` untuk membuka modal
   - Menambahkan handler tombol `1`, `2`, `3` di dalam modal
   - Menambahkan handler `Enter` untuk konfirmasi export
   - Menambahkan handler `↑`/`↓` untuk navigasi di modal
   - Mencegah panel navigation ketika modal terbuka

### State Management

Modal menyimpan pilihan format di field `export_format_selection`:
- `0` = CSV
- `1` = JSON
- `2` = HTML

Pilihan ini digunakan untuk:
1. Menampilkan indikator `→` di modal
2. Menentukan format export ketika `Enter` ditekan

## Contoh Penggunaan

### Scenario 1: Export ke HTML
1. Buka Systrix TUI
2. Tekan `e`
3. Tekan `3` atau gunakan `↓` dua kali
4. Tekan `Enter`
5. File HTML akan di-export

### Scenario 2: Export ke CSV
1. Buka Systrix TUI
2. Tekan `e`
3. Tekan `1`
4. Tekan `Enter`
5. File CSV akan di-export

### Scenario 3: Batalkan Export
1. Buka Systrix TUI
2. Tekan `e`
3. Tekan `ESC`
4. Modal tertutup, tidak ada export

## Benefits

1. **User-Friendly**: Pengguna bisa melihat semua pilihan format sebelum export
2. **Flexible**: Bisa memilih dengan angka atau arrow keys
3. **Clear**: Deskripsi singkat untuk setiap format
4. **Consistent**: Menggunakan modal system yang sama dengan fitur lain
5. **Efficient**: Tetap ada shortcut langsung untuk power users

---

**Status**: ✅ COMPLETE
**Version**: 0.3.0
**Date**: December 9, 2025
