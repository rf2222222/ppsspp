// Copyright (c) 2018- PPSSPP Project.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 2.0 or later versions.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License 2.0 for more details.

// A copy of the GPL 2.0 should have been included with the program.
// If not, see http://www.gnu.org/licenses/

// Official git repository and contact information can be found at
// https://github.com/hrydgard/ppsspp and http://www.ppsspp.org/.

#pragma once

#include "Core/Debugger/WebSocket/WebSocketUtils.h"

DebuggerSubscriber *WebSocketGPUBufferInit(DebuggerEventHandlerMap &map);

void WebSocketGPUBufferScreenshot(DebuggerRequest &req);
void WebSocketGPUBufferRenderColor(DebuggerRequest &req);
void WebSocketGPUBufferRenderDepth(DebuggerRequest &req);
void WebSocketGPUBufferRenderStencil(DebuggerRequest &req);
void WebSocketGPUBufferTexture(DebuggerRequest &req);
void WebSocketGPUBufferClut(DebuggerRequest &req);