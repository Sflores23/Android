// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Provides the Windows Networking WNNC_NET definitions to winnetwk.h and the IFS Kit.
use shared::minwindef::DWORD;
pub const WNNC_NET_MSNET: DWORD = 0x00010000;
pub const WNNC_NET_SMB: DWORD = 0x00020000;
pub const WNNC_NET_NETWARE: DWORD = 0x00030000;
pub const WNNC_NET_VINES: DWORD = 0x00040000;
pub const WNNC_NET_10NET: DWORD = 0x00050000;
pub const WNNC_NET_LOCUS: DWORD = 0x00060000;
pub const WNNC_NET_SUN_PC_NFS: DWORD = 0x00070000;
pub const WNNC_NET_LANSTEP: DWORD = 0x00080000;
pub const WNNC_NET_9TILES: DWORD = 0x00090000;
pub const WNNC_NET_LANTASTIC: DWORD = 0x000A0000;
pub const WNNC_NET_AS400: DWORD = 0x000B0000;
pub const WNNC_NET_FTP_NFS: DWORD = 0x000C0000;
pub const WNNC_NET_PATHWORKS: DWORD = 0x000D0000;
pub const WNNC_NET_LIFENET: DWORD = 0x000E0000;
pub const WNNC_NET_POWERLAN: DWORD = 0x000F0000;
pub const WNNC_NET_BWNFS: DWORD = 0x00100000;
pub const WNNC_NET_COGENT: DWORD = 0x00110000;
pub const WNNC_NET_FARALLON: DWORD = 0x00120000;
pub const WNNC_NET_APPLETALK: DWORD = 0x00130000;
pub const WNNC_NET_INTERGRAPH: DWORD = 0x00140000;
pub const WNNC_NET_SYMFONET: DWORD = 0x00150000;
pub const WNNC_NET_CLEARCASE: DWORD = 0x00160000;
pub const WNNC_NET_FRONTIER: DWORD = 0x00170000;
pub const WNNC_NET_BMC: DWORD = 0x00180000;
pub const WNNC_NET_DCE: DWORD = 0x00190000;
pub const WNNC_NET_AVID: DWORD = 0x001A0000;
pub const WNNC_NET_DOCUSPACE: DWORD = 0x001B0000;
pub const WNNC_NET_MANGOSOFT: DWORD = 0x001C0000;
pub const WNNC_NET_SERNET: DWORD = 0x001D0000;
pub const WNNC_NET_RIVERFRONT1: DWORD = 0x001E0000;
pub const WNNC_NET_RIVERFRONT2: DWORD = 0x001F0000;
pub const WNNC_NET_DECORB: DWORD = 0x00200000;
pub const WNNC_NET_PROTSTOR: DWORD = 0x00210000;
pub const WNNC_NET_FJ_REDIR: DWORD = 0x00220000;
pub const WNNC_NET_DISTINCT: DWORD = 0x00230000;
pub const WNNC_NET_TWINS: DWORD = 0x00240000;
pub const WNNC_NET_RDR2SAMPLE: DWORD = 0x00250000;
pub const WNNC_NET_CSC: DWORD = 0x00260000;
pub const WNNC_NET_3IN1: DWORD = 0x00270000;
pub const WNNC_NET_EXTENDNET: DWORD = 0x00290000;
pub const WNNC_NET_STAC: DWORD = 0x002A0000;
pub const WNNC_NET_FOXBAT: DWORD = 0x002B0000;
pub const WNNC_NET_YAHOO: DWORD = 0x002C0000;
pub const WNNC_NET_EXIFS: DWORD = 0x002D0000;
pub const WNNC_NET_DAV: DWORD = 0x002E0000;
pub const WNNC_NET_KNOWARE: DWORD = 0x002F0000;
pub const WNNC_NET_OBJECT_DIRE: DWORD = 0x00300000;
pub const WNNC_NET_MASFAX: DWORD = 0x00310000;
pub const WNNC_NET_HOB_NFS: DWORD = 0x00320000;
pub const WNNC_NET_SHIVA: DWORD = 0x00330000;
pub const WNNC_NET_IBMAL: DWORD = 0x00340000;
pub const WNNC_NET_LOCK: DWORD = 0x00350000;
pub const WNNC_NET_TERMSRV: DWORD = 0x00360000;
pub const WNNC_NET_SRT: DWORD = 0x00370000;
pub const WNNC_NET_QUINCY: DWORD = 0x00380000;
pub const WNNC_NET_OPENAFS: DWORD = 0x00390000;
pub const WNNC_NET_AVID1: DWORD = 0x003A0000;
pub const WNNC_NET_DFS: DWORD = 0x003B0000;
pub const WNNC_NET_KWNP: DWORD = 0x003C0000;
pub const WNNC_NET_ZENWORKS: DWORD = 0x003D0000;
pub const WNNC_NET_DRIVEONWEB: DWORD = 0x003E0000;
pub const WNNC_NET_VMWARE: DWORD = 0x003F0000;
pub const WNNC_NET_RSFX: DWORD = 0x00400000;
pub const WNNC_NET_MFILES: DWORD = 0x00410000;
pub const WNNC_NET_MS_NFS: DWORD = 0x00420000;
pub const WNNC_NET_GOOGLE: DWORD = 0x00430000;
pub const WNNC_NET_NDFS: DWORD = 0x00440000;
pub const WNNC_NET_DOCUSHARE: DWORD = 0x00450000;
pub const WNNC_CRED_MANAGER: DWORD = 0xFFFF0000;
pub const WNNC_NET_LANMAN: DWORD = WNNC_NET_SMB;
