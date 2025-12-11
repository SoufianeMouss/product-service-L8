use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 55\" 4K UHD Smart TV".to_string(),
            price: 699.99,
            description: "Experience stunning picture quality with HDR, Crystal Display technology, and built-in streaming apps like Netflix and Disney+.".to_string(),
            image: "/products/1.png".to_string(),
        },
        Product {
            id: 2,
            name: "LG 65\" OLED C3 Smart TV".to_string(),
            price: 1899.99,
            description: "OLED technology delivers perfect blacks and vivid colors for cinematic viewing. Supports Dolby Atmos and Dolby Vision.".to_string(),
            image: "/products/2.png".to_string(),
        },
        Product {
            id: 3,
            name: "Dell XPS 13 Laptop (i7, 16GB RAM, 512GB SSD)".to_string(),
            price: 1299.99,
            description: "Ultra-portable 13-inch laptop with slim bezels, excellent battery life, and premium build quality. Perfect for students and professionals.".to_string(),
            image: "/products/3.png".to_string(),
        },
        Product {
            id: 4,
            name: "Apple MacBook Air 13\" M2".to_string(),
            price: 1499.99,
            description: "Powered by Apple's M2 chip with blazing-fast performance, 13.6\" Liquid Retina display, and all-day battery life.".to_string(),
            image: "/products/4.png".to_string(),
        },
        Product {
            id: 5,
            name: "iPhone 15 Pro (128GB)".to_string(),
            price: 1399.99,
            description: "The latest iPhone with A17 Pro chip, titanium design, advanced camera system, and USB-C connectivity.".to_string(),
            image: "/products/5.png".to_string(),
        },
        Product {
            id: 6,
            name: "Samsung Galaxy S24 (256GB)".to_string(),
            price: 1199.99,
            description: "Features a 6.4\" AMOLED display, powerful triple camera, AI Zoom, and long-lasting battery life.".to_string(),
            image: "/products/6.png".to_string(),
        },
        Product {
            id: 7,
            name: "OnePlus 12".to_string(),
            price: 1199.99,
            description: "A performance-focused flagship with Snapdragon 8 Gen 3, 5400mAh battery, 120Hz AMOLED display, and Hasselblad-tuned cameras for stunning photos.".to_string(),
            image: "/products/7.png".to_string(),
        },

        Product {
            id: 8,
            name: "PlayStation 5 Console".to_string(),
            price: 699.99,
            description: "The latest gaming console from Sony with ultra-fast SSD, ray tracing, and immersive gaming experiences.".to_string(),
            image: "/products/8.png".to_string(),
        },
        Product {
            id: 9,
            name: "Nintendo Switch OLED Edition".to_string(),
            price: 449.99,
            description: "Vibrant OLED screen, enhanced audio, and versatile play modes. Perfect for gaming on the go or at home.".to_string(),
            image: "/products/9.png".to_string(),
        },
        Product {
            id: 10,
            name: "Apple Watch Series 9".to_string(),
            price: 599.99,
            description: "Advanced health features, high-brightness display, and fast performance powered by the S9 chip.".to_string(),
            image: "/products/10.png".to_string(),
        },
    ]
}