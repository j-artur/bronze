use std::ptr::null_mut;
use winapi::shared::{dxgi::*, dxgiformat::*, dxgitype::*, winerror::*};
use winapi::um::{d3d11::*, d3dcommon::*};
use winapi::Interface;

use super::window::{Window, WindowMode::*};
use super::Color;

pub struct Graphics {
    swap_chain: *mut IDXGISwapChain,
    render_target_view: *mut ID3D11RenderTargetView,
    blend_state: *mut ID3D11BlendState,
    feature_level: D3D_FEATURE_LEVEL,
    bg_color: [f32; 4],
    vsync: bool,
    device: *mut ID3D11Device,
    context: *mut ID3D11DeviceContext,
    viewport: D3D11_VIEWPORT,
}

impl Graphics {
    pub fn new() -> Self {
        Self {
            swap_chain: null_mut(),
            render_target_view: null_mut(),
            blend_state: null_mut(),
            feature_level: D3D_FEATURE_LEVEL_11_0,
            bg_color: [0.0, 0.0, 0.0, 0.0],
            vsync: false,
            device: null_mut(),
            context: null_mut(),
            viewport: D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: 0.0,
                Height: 0.0,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            },
        }
    }

    pub fn initialize(&mut self, window: &Window) -> bool {
        // DirectX Device

        let create_device_flags = if cfg!(debug_assertions) {
            D3D11_CREATE_DEVICE_DEBUG
        } else {
            0
        };

        unsafe {
            if FAILED(D3D11CreateDevice(
                null_mut(),
                D3D_DRIVER_TYPE_HARDWARE,
                null_mut(),
                create_device_flags,
                null_mut(),
                0,
                D3D11_SDK_VERSION,
                &mut self.device,
                &mut self.feature_level,
                &mut self.context,
            )) {
                if FAILED(D3D11CreateDevice(
                    null_mut(),
                    D3D_DRIVER_TYPE_WARP,
                    null_mut(),
                    create_device_flags,
                    null_mut(),
                    0,
                    D3D11_SDK_VERSION,
                    &mut self.device,
                    &mut self.feature_level,
                    &mut self.context,
                )) {
                    return false;
                }

                println!(
                "WARNING: Direct3D hardware device not available, using software device instead."
            );
            }

            // Backgroud Color

            let Color(r, g, b, _) = window.bg();

            self.bg_color[0] = r as f32 / 255.0;
            self.bg_color[1] = g as f32 / 255.0;
            self.bg_color[2] = b as f32 / 255.0;
            self.bg_color[3] = 1.0;

            // DXGI Interfaces

            let mut dxgi_device: *mut IDXGIDevice = null_mut();
            if FAILED(
                (*self.device)
                    .QueryInterface(&IDXGIDevice::uuidof(), &mut dxgi_device as *mut _ as *mut _),
            ) {
                return false;
            }

            let mut dxgi_adapter: *mut IDXGIAdapter = null_mut();
            if FAILED((*dxgi_device).GetParent(
                &IDXGIAdapter::uuidof(),
                &mut dxgi_adapter as *mut _ as *mut _,
            )) {
                return false;
            }

            let mut dxgi_factory: *mut IDXGIFactory = null_mut();
            if FAILED((*dxgi_adapter).GetParent(
                &IDXGIFactory::uuidof(),
                &mut dxgi_factory as *mut _ as *mut _,
            )) {
                return false;
            }

            // Swap Chain

            let mut swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
                BufferDesc: DXGI_MODE_DESC {
                    Width: window.width() as u32,
                    Height: window.height() as u32,
                    RefreshRate: DXGI_RATIONAL {
                        Numerator: 60,
                        Denominator: 1,
                    },
                    Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
                },
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 2,
                OutputWindow: window.handle(),
                Windowed: (window.mode() != Fullscreen) as i32,
                SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
            };

            if FAILED((*dxgi_factory).CreateSwapChain(
                self.device as *mut _,
                &mut swap_chain_desc,
                &mut self.swap_chain,
            )) {
                return false;
            }

            if FAILED((*dxgi_factory).MakeWindowAssociation(window.handle(), 1 << 1)) {
                return false;
            }

            // Render Target View

            let mut back_buffer: *mut ID3D11Texture2D = null_mut();
            if FAILED((*self.swap_chain).GetBuffer(
                0,
                &ID3D11Texture2D::uuidof(),
                &mut back_buffer as *mut _ as *mut _,
            )) {
                return false;
            }

            if FAILED((*self.device).CreateRenderTargetView(
                back_buffer as *mut _,
                null_mut(),
                &mut self.render_target_view,
            )) {
                return false;
            }

            (*self.context).OMSetRenderTargets(1, &mut self.render_target_view, null_mut());

            // Viewport / Rasterizer

            self.viewport.Width = window.width() as f32;
            self.viewport.Height = window.height() as f32;

            (*self.context).RSSetViewports(1, &self.viewport);

            // Blend State

            let blend_state_desc = D3D11_BLEND_DESC {
                AlphaToCoverageEnable: 0,
                IndependentBlendEnable: 0,
                RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC {
                    BlendEnable: 1,
                    SrcBlend: D3D11_BLEND_SRC_ALPHA,
                    DestBlend: D3D11_BLEND_INV_SRC_ALPHA,
                    BlendOp: D3D11_BLEND_OP_ADD,
                    SrcBlendAlpha: D3D11_BLEND_ONE,
                    DestBlendAlpha: D3D11_BLEND_ZERO,
                    BlendOpAlpha: D3D11_BLEND_OP_ADD,
                    RenderTargetWriteMask: D3D11_COLOR_WRITE_ENABLE_ALL as u8,
                }; 8],
            };

            if FAILED((*self.device).CreateBlendState(&blend_state_desc, &mut self.blend_state)) {
                return false;
            }

            // Release Interfaces

            (*dxgi_device).Release();
            (*dxgi_adapter).Release();
            (*dxgi_factory).Release();
            (*back_buffer).Release();
        }

        true
    }

    pub fn use_vsync(&mut self, vsync: bool) {
        self.vsync = vsync;
    }

    pub fn clear(&mut self) {
        unsafe {
            (*self.context).ClearRenderTargetView(self.render_target_view, &self.bg_color);
        }
    }

    pub fn present(&mut self) {
        unsafe {
            (*self.swap_chain).Present(self.vsync as u32, 0);
            (*self.context).OMSetRenderTargets(1, &self.render_target_view, null_mut());
        }
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe {
            if !self.blend_state.is_null() {
                (*self.blend_state).Release();
                self.blend_state = null_mut();
            }

            if !self.render_target_view.is_null() {
                (*self.render_target_view).Release();
                self.render_target_view = null_mut();
            }

            if !self.swap_chain.is_null() {
                (*self.swap_chain).SetFullscreenState(0, null_mut());
                (*self.swap_chain).Release();
                self.swap_chain = null_mut();
            }

            if !self.context.is_null() {
                (*self.context).ClearState();
                (*self.context).Release();
                self.context = null_mut();
            }

            if !self.device.is_null() {
                (*self.device).Release();
                self.device = null_mut();
            }
        }
    }
}
