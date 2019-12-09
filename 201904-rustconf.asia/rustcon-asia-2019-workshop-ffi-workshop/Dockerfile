FROM debian:9
ADD https://www.zlib.net/zlib-1.2.11.tar.gz /root/zlib-1.2.11.tar.gz
ADD https://github.com/webmproject/libwebp/archive/v1.0.2.tar.gz /root/libwebp-1.0.2.tar.gz 
ADD https://github.com/glennrp/libpng/archive/v1.6.35.tar.gz  /root/libpng-1.6.35.tar.gz 
ENV PATH=/root/.cargo/bin:$PATH
RUN apt-get install && \
    apt-get update && apt-get upgrade -y && \
    apt-get install build-essential cmake nasm libssl-dev pkg-config wget ca-certificates -y 

RUN cd /root && tar -xvf zlib-1.2.11.tar.gz && \
    tar -xvf libwebp-1.0.2.tar.gz  && \
    tar -xvf libpng-1.6.35.tar.gz  && cd zlib-1.2.11 && ./configure && make -j4 install && \
    cd /root/libpng-1.6.35 && ./configure && make install && cd /root/libwebp-1.0.2 && mkdir build && cd build && cmake ../ && make -j4 install  && rm -rf zlib* libwebp* libjpeg* libpng* && \
    apt-get install -y clang libclang-dev && \
    RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup  curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain nightly -y

