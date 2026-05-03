# Hybrid Oracle Client (Rust + Solidity)

Este proyecto es un cliente híbrido que consulta el precio de ETH/USD en tiempo real desde la red de pruebas Sepolia utilizando oráculos de Chainlink.

## Estructura del Proyecto
- `/contracts`: Contiene el contrato `PriceConsumer.sol` desplegado en Sepolia.
- `/src`: Cliente desarrollado en Rust utilizando la librería **Alloy** para comunicación asíncrona vía RPC.

## Tecnologías
- **Solidity**: Contrato inteligente on-chain.
- **Rust**: Cliente off-chain de alto rendimiento.
- **Alloy & Tokio**: Manejo de red y asincronía.
- **Chainlink**: Oráculo de datos descentralizado.

## Cómo usar
1. Configura tu `.env` con `RPC_URL` y `CONTRACT_ADDRESS`.
2. Ejecuta `cargo run`.

## Paragraph
Puedes tambien leer sobre el documento aqui:
https://paragraph.com/@gabrielnavarro/building-a-hybrid-smart-contract-with-rust-and-chainlink
