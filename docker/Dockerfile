# مرحلة البناء
FROM rust:1.82-slim AS builder

WORKDIR /app

# تثبيت متطلبات البناء
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# نسخ ملفات المشروع
COPY . .

# بناء التطبيق
RUN cargo build --release

# مرحلة التشغيل
FROM debian:bookworm-slim AS final

# تثبيت متطلبات التشغيل
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# نسخ الملف التنفيذي
COPY --from=builder /app/target/release/rust-api ./rust-api

# إنشاء مستخدم غير جذر
RUN useradd -r -s /bin/false apiuser
RUN chown apiuser:apiuser ./rust-api
USER apiuser

EXPOSE 8002

CMD ["./rust-api"]