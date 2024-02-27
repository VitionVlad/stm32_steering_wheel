if (!navigator.gpu) {
    throw new Error("WebGPU not supported on this browser.");
}

const adapter = await navigator.gpu.requestAdapter();
if (!adapter) {
  throw new Error("No appropriate GPUAdapter found.");
}

const device = await adapter.requestDevice();

export class Gfxrender{
    constructor(canvasid, renderscale, shadowmapres){
        this.rscale = renderscale;
        this.shadowr = shadowmapres;
        this.canvas = document.getElementById(canvasid);
        this.canvas.width = this.canvas.offsetWidth;
        this.canvas.height = this.canvas.offsetHeight;
        this.context = this.canvas.getContext("webgpu");
        this.canvasFormat = navigator.gpu.getPreferredCanvasFormat();
        this.context.configure({
          device: device,
          format: this.canvasFormat,
        });
        console.log("Gfxrender: canvas resolution is: " + this.canvas.width + " " + this.canvas.height);
        this.depthTexture = [
            device.createTexture({
                label: "depth1",
                format: "depth24plus",
                size: [this.canvas.width, this.canvas.height],
                usage: GPUTextureUsage.RENDER_ATTACHMENT,
            }),
            device.createTexture({
                label: "depth2",
                format: "depth24plus",
                size: [this.canvas.width, this.canvas.height],
                usage: GPUTextureUsage.RENDER_ATTACHMENT,
            })
        ];
        this.oldresx = this.canvas.width;
        this.oldresy = this.canvas.height;
        this.mainPassTexture = [
            device.createTexture({
                label: "main1",
                format: navigator.gpu.getPreferredCanvasFormat(),
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            }),
            device.createTexture({
                label: "main2",
                format: navigator.gpu.getPreferredCanvasFormat(),
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            })
        ];
        this.mainPassDepthTexture = [
            device.createTexture({
                label: "maindepth1",
                format: "depth24plus",
                size: [Number(this.canvas.width*this.rscale), Number(this.canvas.height*this.rscale)],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            }),
            device.createTexture({
                label: "maindepth2",
                format: "depth24plus",
                size: [Number(this.canvas.width*this.rscale), Number(this.canvas.height*this.rscale)],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            })
        ];
        this.shadowTexture = [
            device.createTexture({
                label: "shadow1",
                format: "depth24plus",
                size: [this.shadowr, this.shadowr],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            }),
            device.createTexture({
                label: "shadow2",
                format: "depth24plus",
                size: [this.shadowr, this.shadowr],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            })
        ];
        this.encoder = device.createCommandEncoder();
        this.isshadowpass = false;
        this.inpost = false;
        this.passbeg = false;
        this.currentworkingbuffers = false;
        this.currentworkingbufferssh = false;
        this.change = false;
        this.changesh = false;
    }
    gfxgetcanvassizex(){
        return this.canvas.width;
    }
    gfxgetcanvassizey(){
        return this.canvas.height;
    }
    gfxsetrenderscale(renderscale){
        this.rscale = renderscale;
        this.change = true;
    }
    gfxsetshadowmapres(shadowmapres){
        this.shadowr = shadowmapres;
        this.shadowTexture[Number(!this.currentworkingbufferssh)] = device.createTexture({
            format: "depth24plus",
            size: [this.shadowr, this.shadowr],
            usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
        });
        this.changesh = true;
    }
    gfxbeginpass(lop, dlop){
        this.passbeg = true;
        this.inpost = true;
        this.isshadowpass = false;
        this.pass = this.encoder.beginRenderPass({
            colorAttachments: [{
               view: this.context.getCurrentTexture().createView(),
               clearValue: { r: 0, g: 0, b: 0, a: 1 },
               loadOp: lop,
               storeOp: "store",
            }],
            depthStencilAttachment: {
                view: this.depthTexture[Number(this.currentworkingbuffers)].createView(),
                depthClearValue: 1.0,
                depthLoadOp: dlop,
                depthStoreOp: "store",
            }
        });
    }
    gfxbeginmainpass(lop, dlop){
        this.passbeg = true;
        this.inpost = false;
        this.isshadowpass = false;
        this.pass = this.encoder.beginRenderPass({
            colorAttachments: [{
               view: this.mainPassTexture[Number(this.currentworkingbuffers)].createView(),
               clearValue: { r: 0, g: 0, b: 0, a: 1 },
               loadOp: lop,
               storeOp: "store",
            }],
            depthStencilAttachment: {
                view: this.mainPassDepthTexture[Number(this.currentworkingbuffers)].createView(),
                depthClearValue: 1.0,
                depthLoadOp: dlop,
                depthStoreOp: "store",
            }
        });
    }
    gfxbeginshadowpass(dlop){
        this.passbeg = true;
        this.inpost = false;
        this.isshadowpass = true;
        this.pass = this.encoder.beginRenderPass({
            colorAttachments: [],
            depthStencilAttachment: {
                view: this.shadowTexture[Number(this.currentworkingbufferssh)].createView(),
                depthClearValue: 1.0,
                depthLoadOp: dlop,
                depthStoreOp: "store",
            }
        });
    }
    gfxendpass(){
        if(this.passbeg){
            this.pass.end();
            this.passbeg = false;
        }
    }
    gfxfinishrender(){
        device.queue.submit([this.encoder.finish()]);
        this.encoder = device.createCommandEncoder();
        this.canvas.width = this.canvas.offsetWidth;
        this.canvas.height = this.canvas.offsetHeight;

        if(this.oldresx != this.canvas.width || this.oldresy != this.canvas.height || this.change){
            console.log("Gfxrender: changing working buffers from " + Number(this.currentworkingbuffers) + " to " + Number(!this.currentworkingbuffers));
            this.depthTexture[Number(!this.currentworkingbuffers)].destroy();
            this.depthTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "d",
                format: "depth24plus",
                size: [this.canvas.width, this.canvas.height],
                usage: GPUTextureUsage.RENDER_ATTACHMENT,
            });
            this.mainPassTexture[Number(this.currentworkingbuffers)].destroy();
            this.mainPassTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "m",
                format: navigator.gpu.getPreferredCanvasFormat(),
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            });
            this.mainPassDepthTexture[Number(!this.currentworkingbuffers)].destroy();
            this.mainPassDepthTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "md",
                format: "depth24plus",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale],
                usage:  GPUTextureUsage.TEXTURE_BINDING |  GPUTextureUsage.RENDER_ATTACHMENT,
            });
            this.currentworkingbuffers = !this.currentworkingbuffers;
            console.log("Gfxrender: canvas resized from: x="+this.oldresx+" to x="+this.canvas.width+" from y="+this.oldresy+" to y="+this.canvas.height);
            this.oldresx = this.canvas.width;
            this.oldresy = this.canvas.height;
            this.change = false
        }
        if(this.changesh){
            this.currentworkingbufferssh = !this.currentworkingbufferssh;
            this.changesh = false;
        }
    }
}

export class Gfxmesh{
    preparesh(shadowvertexcode){
        this.vertexshadercode = device.createShaderModule({
            code: shadowvertexcode
        });
        const shadowbindGroupLayout = device.createBindGroupLayout({
            entries: [
              {
                binding: 0,
                visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                buffer: {},
              },
            ],
        });
        this.shadowpipeline = device.createRenderPipeline({
            layout: device.createPipelineLayout({
                bindGroupLayouts: [shadowbindGroupLayout],
            }),
            vertex: {
              module: this.vertexshadercode,
              entryPoint: "vertexMain",
              buffers: [
                this.vertexBufferLayout,
            ]
            },
            depthStencil: {
                depthWriteEnabled: true,
                depthCompare: 'less',
                format: 'depth24plus',
            },
        });
        this.sbindGroup = device.createBindGroup({
            layout: this.shadowpipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffer 
                    }
                },
            ],
        });
    }
    preparemainrender(vertexcode, fragmentcode, texid, gfx, magfilter, minfilter){
        this.vertexcode = device.createShaderModule({
            code: vertexcode
        });
        this.fragmentcode = device.createShaderModule({
            code: fragmentcode
        });
        const bindGroupLayout = device.createBindGroupLayout({
            label: "mainGroupLayout",
            entries: [
              {
                binding: 0,
                visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                buffer: {},
              },
              {
                binding: 1,
                visibility: GPUShaderStage.FRAGMENT,
                sampler: {},
              },
              {
                binding: 2,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    viewDimension: "2d-array",
                },
              },
              {
                binding: 3,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    sampleType: 'depth',
                  },
              },
            ],
          });
        this.pipeline = device.createRenderPipeline({
            label: "mainPipeline",
            layout: device.createPipelineLayout({
                bindGroupLayouts: [bindGroupLayout],
            }),
            vertex: {
              module: this.vertexcode,
              entryPoint: "vertexMain",
              buffers: [
                this.vertexBufferLayout,
                this.uvBufferLayout,
                this.nBufferLayout,
            ]
            },
            fragment: {
              module: this.fragmentcode,
              entryPoint: "fragmentMain",
              targets: [{
                format: gfx.canvasFormat
              }]
            },
            depthStencil: {
                depthWriteEnabled: true,
                depthCompare: 'less',
                format: 'depth24plus',
            },
        });
        const ids = texid.split(";");
        this.colortex = device.createTexture({
            label: "colorTex",
            size: [document.getElementById(ids[0]).width, document.getElementById(ids[0]).height, ids.length+1],
            dimension: "2d",
            format: 'rgba8unorm',
            usage:
              GPUTextureUsage.TEXTURE_BINDING |
              GPUTextureUsage.COPY_DST |
              GPUTextureUsage.RENDER_ATTACHMENT,
        });
        this.sampler = device.createSampler({
            magFilter: magfilter,
            minFilter: minfilter,
        });
        for(let i = 0; i < ids.length; i++){
            device.queue.copyExternalImageToTexture(
                { source: document.getElementById(ids[i]) },
                { 
                    texture: this.colortex,
                    origin: [0, 0, i]
                },
                [document.getElementById(ids[i]).width, document.getElementById(ids[i]).height]
            );
        }
        this.bindGroup = device.createBindGroup({
            label: "mainBindGroup",
            layout: this.pipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffer 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: this.colortex.createView()
                },
                {
                    binding: 3,
                    resource: gfx.shadowTexture[Number(gfx.currentworkingbufferssh)].createView()
                },
            ],
        });
    }
    preparpostrender(vertexcode, fragmentcode, texid, gfx, magfilter, minfilter){
        this.vertexcode = device.createShaderModule({
            code: vertexcode
        });
        this.fragmentcode = device.createShaderModule({
            code: fragmentcode
        });
        const bindpostGroupLayout = device.createBindGroupLayout({
            entries: [
              {
                binding: 0,
                visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                buffer: {},
              },
              {
                binding: 1,
                visibility: GPUShaderStage.FRAGMENT,
                sampler: {},
              },
              {
                binding: 2,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    viewDimension: "2d-array",
                },
              },
              {
                binding: 3,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    sampleType: 'depth',
                  },
              },
              {
                binding: 4,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {},
              },
              {
                binding: 5,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    sampleType: 'depth',
                  },
              },
            ],
          });
        const ids = texid.split(";");
        this.colortex = device.createTexture({
            label: "shaderposttexture",
            size: [document.getElementById(ids[0]).width, document.getElementById(ids[0]).height, ids.length+1],
            dimension: "2d",
            format: 'rgba8unorm',
            usage:
              GPUTextureUsage.TEXTURE_BINDING |
              GPUTextureUsage.COPY_DST |
              GPUTextureUsage.RENDER_ATTACHMENT,
        });
        this.sampler = device.createSampler({
            magFilter: magfilter,
            minFilter: minfilter,
        });
        for(let i = 0; i < ids.length; i++){
            device.queue.copyExternalImageToTexture(
                { source: document.getElementById(ids[i]) },
                { 
                    texture: this.colortex,
                    origin: [0, 0, i]
                },
                [document.getElementById(ids[i]).width, document.getElementById(ids[i]).height]
            );
        }
        this.postpipeline = device.createRenderPipeline({
            layout: device.createPipelineLayout({
                bindGroupLayouts: [bindpostGroupLayout],
            }),
            vertex: {
              module: this.vertexcode,
              entryPoint: "vertexMain",
              buffers: [
                this.vertexBufferLayout,
                this.uvBufferLayout,
                this.nBufferLayout,
            ]
            },
            fragment: {
              module: this.fragmentcode,
              entryPoint: "fragmentMain",
              targets: [{
                format: gfx.canvasFormat
              }]
            },
            depthStencil: {
                depthWriteEnabled: true,
                depthCompare: 'less',
                format: 'depth24plus',
            },
        });
        this.postbindGroup = device.createBindGroup({
            layout: this.postpipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffer 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: this.colortex.createView()
                },
                {
                    binding: 3,
                    resource: gfx.shadowTexture[Number(gfx.currentworkingbufferssh)].createView()
                },
                {
                    binding: 4,
                    resource: gfx.mainPassTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 5,
                    resource: gfx.mainPassDepthTexture[Number(gfx.currentworkingbuffers)].createView()
                },
            ],
        });
    }
    constructor(gfx, vertices, uv, normals, lenght, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, magfilter, minfilter, forpost){
        this.forpost = forpost;
        this.lenght = lenght;
        this.ubol = ubol;
        this.uniformBuffer = device.createBuffer({
            size: ubol,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        });
        this.vertexBuffer = device.createBuffer({
            size: 16*lenght,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        this.uvBuffer = device.createBuffer({
            size: 8*lenght,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        this.nBuffer = device.createBuffer({
            size: 12*lenght,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        device.queue.writeBuffer(this.vertexBuffer, 0, vertices);
        device.queue.writeBuffer(this.uvBuffer, 0, uv);
        device.queue.writeBuffer(this.nBuffer, 0, normals);
        this.vertexBufferLayout = {
            arrayStride: 16,
            attributes: [{
              format: "float32x4",
              offset: 0,
              shaderLocation: 0,
            }],
        };
        this.uvBufferLayout = {
            arrayStride: 8,
            attributes: [{
              format: "float32x2",
              offset: 0,
              shaderLocation: 1,
            }],
        };
        this.nBufferLayout = {
            arrayStride: 12,
            attributes: [{
              format: "float32x3",
              offset: 0,
              shaderLocation: 2,
            }],
        };
        if(!forpost){
            this.preparemainrender(vertexcode, fragmentcode, texid, gfx, magfilter, minfilter);
        }else{
            this.preparpostrender(vertexcode, fragmentcode, texid, gfx, magfilter, minfilter);
        }
        this.preparesh(shadowvertexcode);
    }
    recpostg(gfx){
        this.postbindGroup = device.createBindGroup({
            layout: this.postpipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffer 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: this.colortex.createView()
                },
                {
                    binding: 3,
                    resource: gfx.shadowTexture[Number(gfx.currentworkingbufferssh)].createView()
                },
                {
                    binding: 4,
                    resource: gfx.mainPassTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 5,
                    resource: gfx.mainPassDepthTexture[Number(gfx.currentworkingbuffers)].createView()
                },
            ],
        });
    }
    recg(gfx){
        this.bindGroup = device.createBindGroup({
            label: "mainBindGroup",
            layout: this.pipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffer 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: this.colortex.createView()
                },
                {
                    binding: 3,
                    resource: gfx.shadowTexture[Number(gfx.currentworkingbufferssh)].createView()
                },
            ],
        });
    }
    draw(gfx, uniformValues){
        device.queue.writeBuffer(this.uniformBuffer, 0, uniformValues);
        if(gfx.isshadowpass){
            gfx.pass.setPipeline(this.shadowpipeline);
            gfx.pass.setBindGroup(0, this.sbindGroup);
            gfx.pass.setVertexBuffer(0, this.vertexBuffer);
        }else{
            if(gfx.inpost && this.forpost){
                this.recpostg(gfx);
                gfx.pass.setPipeline(this.postpipeline);
                gfx.pass.setBindGroup(0, this.postbindGroup);
                gfx.pass.setVertexBuffer(0, this.vertexBuffer);
                gfx.pass.setVertexBuffer(1, this.uvBuffer);
                gfx.pass.setVertexBuffer(2, this.nBuffer);
            }
            if (!gfx.inpost && !this.forpost){
                this.recg(gfx);
                gfx.pass.setPipeline(this.pipeline);
                gfx.pass.setBindGroup(0, this.bindGroup);
                gfx.pass.setVertexBuffer(0, this.vertexBuffer);
                gfx.pass.setVertexBuffer(1, this.uvBuffer);
                gfx.pass.setVertexBuffer(2, this.nBuffer);
            }
        }
        gfx.pass.draw(this.lenght);
    }
}

export class Gpucompute{
    constructor(ibs, obs, code){
        this.is = ibs*4;
        this.os = obs*4;
        const module = device.createShaderModule({
            code: code
        });

        const bindGroupLayout = device.createBindGroupLayout({
            label: "compute group layout",
            entries: [{
              binding: 0,
              visibility: GPUShaderStage.COMPUTE,
              buffer: { type: "read-only-storage"}
            }, {
              binding: 1,
              visibility: GPUShaderStage.COMPUTE,
              buffer: { type: "storage"}
            }]
          });

        this.pipeline = device.createComputePipeline({
            label: 'compute pipeline',
            layout: device.createPipelineLayout({
                bindGroupLayouts: [bindGroupLayout],
            }),
            compute: {
              module: module,
              entryPoint: 'computeMain',
            },
        });

        this.inbuf = device.createBuffer({
            size: ibs*4, 
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
        });

        this.outbuf = device.createBuffer({
            size: obs*4, 
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC
        });

        this.outbufcpu = device.createBuffer({
            size: obs*4, 
            mappedAtCreation: false,
            usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
        });

        this.bindGroup = device.createBindGroup({
            layout: this.pipeline.getBindGroupLayout(0),
            entries: [{ 
                binding: 0, 
                resource: { 
                    buffer: this.inbuf 
                }
            },{ 
                binding: 1, 
                resource: { 
                    buffer: this.outbuf 
                }
            },
            ],
        });
        this.rsoutbuf = new Float32Array(this.obs);
        this.ended = true;
    }
    async execute(inbuf, workgroupcount){
        this.ended = false;
        device.queue.writeBuffer(this.inbuf, 0, inbuf);
        const encoder = device.createCommandEncoder();
        const computePass = encoder.beginComputePass();

        computePass.setPipeline(this.pipeline);
        computePass.setBindGroup(0, this.bindGroup);
        computePass.dispatchWorkgroups(workgroupcount);
        computePass.end();

        encoder.copyBufferToBuffer(this.outbuf, 0, this.outbufcpu, 0, this.os);
        const commandBuffer = encoder.finish();
        device.queue.submit([commandBuffer]);

        await this.outbufcpu.mapAsync(GPUMapMode.READ);

        this.rsoutbuf = new Float32Array(this.outbufcpu.getMappedRange(0, this.os).slice());
        this.outbufcpu.unmap();
        this.ended = true;
    }
    getstate(){
        return this.ended;
    }
    getresult(){
        return this.rsoutbuf;
    }
}