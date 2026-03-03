namespace StockStatsApi;

public static class SeedData
{
    public static Dealer[] GetDealers()
    {
        return
        [
            new(1001, "Kadıköy Merkez", "İstanbul",
            [
                new(1, "Envidya Cortex 1000", 10),
                new(2, "Envidya GTX 2000", 5),
                new(3, "Intel Core i9 12900K", 15),
                new(4, "AMDe Rayzen 98 5950X", 8),
                new(5, "32GB RAM DDR5", 12),
                new(6, "ElCi UHD Monitor 27\"", 7),
                new(7, "Samsung SSD 1TB", 20)
            ]),

            new(1002, "Ankara Merkez", "Ankara",
            [
                new(1, "ElCi UHD Monitor 24\"", 8),
                new(2, "32GB RAM DDR5", 15),
                new(3, "16GB RAM DDR4", 25),
                new(4, "Intel Core i7 13700K", 10),
                new(5, "Envidya Cortex 1000", 6),
                new(6, "Samsung SSD 2TB", 12),
                new(7, "Logitech MX Master 3", 30)
            ]),

            new(1003, "Şişli", "İstanbul",
            [
                new(1, "Envidya GTX 2000", 3),
                new(2, "AMDe Rayzen 77 7800X3D", 7),
                new(3, "32GB RAM DDR5", 10),
                new(4, "ElCi UHD Monitor 32\"", 5),
                new(5, "Corsair Power Supply 850W", 15),
                new(6, "NZXT Kraken X63", 8)
            ]),

            new(1004, "İzmir Konak", "İzmir",
            [
                new(1, "ElCi UHD Monitor 24\"", 12),
                new(2, "ElCi UHD Monitor 27\"", 9),
                new(3, "16GB RAM DDR4", 18),
                new(4, "8GB RAM DDR4", 25),
                new(5, "Intel Core i5 13600K", 14),
                new(6, "Envidya Cortex 1000", 11),
                new(7, "Kingston SSD 512GB", 22)
            ]),

            new(1005, "Bursa Nilüfer", "Bursa",
            [
                new(1, "Envidya GTX 2000", 8),
                new(2, "32GB RAM DDR5", 6),
                new(3, "ElCi UHD Monitor 32\"", 4),
                new(4, "AMDe Rayzen 98 5950X", 5),
                new(5, "MSI Motherboard X670", 7),
                new(6, "Western Digital HDD 4TB", 10)
            ]),

            new(1006, "Antalya Lara", "Antalya",
            [
                new(1, "ElCi UHD Monitor 24\"", 15),
                new(2, "Logitech G502 Mouse", 20),
                new(3, "Corsair K95 Keyboard", 12),
                new(4, "Intel Core i9 12900K", 6),
                new(5, "Samsung SSD 1TB", 14)
            ]),

            new(1007, "Adana Seyhan", "Adana",
            [
                new(1, "32GB RAM DDR5", 9),
                new(2, "16GB RAM DDR4", 16),
                new(3, "ElCi UHD Monitor 27\"", 11),
                new(4, "Envidya Cortex 1000", 13),
                new(5, "AMDe Rayzen 77 7800X3D", 4)
            ]),

            new(1008, "Gaziantep Şahinbey", "Gaziantep",
            [
                new(1, "ElCi UHD Monitor 24\"", 7),
                new(2, "32GB RAM DDR5", 5),
                new(3, "Envidya GTX 2000", 12),
                new(4, "Intel Core i7 13700K", 8),
                new(5, "Cooler Master Case", 10)
            ]),

            new(1009, "Konya Selçuklu", "Konya",
            [
                new(1, "ElCi UHD Monitor 32\"", 6),
                new(2, "ElCi UHD Monitor 27\"", 14),
                new(3, "16GB RAM DDR4", 20),
                new(4, "AMDe Rayzen 98 5950X", 9),
                new(5, "Seasonic Power Supply 1000W", 7)
            ]),

            new(1010, "Kayseri Melikgazi", "Kayseri",
            [
                new(1, "Envidya Cortex 1000", 10),
                new(2, "Intel Core i5 13600K", 15),
                new(3, "Samsung SSD 512GB", 18),
                new(4, "Logitech C920 Webcam", 12)
            ]),

            new(1011, "Eskişehir Odunpazarı", "Eskişehir",
            [
                new(1, "32GB RAM DDR5", 11),
                new(2, "ElCi UHD Monitor 24\"", 13),
                new(3, "Envidya GTX 2000", 7),
                new(4, "AMDe Rayzen 77 7800X3D", 6),
                new(5, "ASUS Motherboard B550", 8)
            ]),

            new(1012, "Diyarbakır Bağlar", "Diyarbakır",
            [
                new(1, "ElCi UHD Monitor 27\"", 10),
                new(2, "16GB RAM DDR4", 22),
                new(3, "Intel Core i9 12900K", 5),
                new(4, "Kingston SSD 1TB", 16)
            ]),

            new(1013, "Samsun İlkadım", "Samsun",
            [
                new(1, "Envidya GTX 2000", 9),
                new(2, "32GB RAM DDR5", 14),
                new(3, "ElCi UHD Monitor 32\"", 8),
                new(4, "Corsair Vengeance RGB", 17),
                new(5, "AMDe Rayzen 98 5950X", 6)
            ]),

            new(1014, "Trabzon Ortahisar", "Trabzon",
            [
                new(1, "ElCi UHD Monitor 24\"", 11),
                new(2, "Intel Core i7 13700K", 9),
                new(3, "Samsung SSD 2TB", 8),
                new(4, "Logitech G733 Headset", 15)
            ]),

            new(1015, "Mersin Akdeniz", "Mersin",
            [
                new(1, "32GB RAM DDR5", 13),
                new(2, "ElCi UHD Monitor 27\"", 12),
                new(3, "Envidya Cortex 1000", 14),
                new(4, "AMDe Rayzen 77 7800X3D", 10),
                new(5, "Western Digital SSD 1TB", 19)
            ]),

            new(1016, "Denizli Pamukkale", "Denizli",
            [
                new(1, "ElCi UHD Monitor 32\"", 7),
                new(2, "16GB RAM DDR4", 24),
                new(3, "8GB RAM DDR4", 30),
                new(4, "Intel Core i5 13600K", 12)
            ]),

            new(1017, "Malatya Yeşilyurt", "Malatya",
            [
                new(1, "Envidya GTX 2000", 6),
                new(2, "ElCi UHD Monitor 24\"", 9),
                new(3, "Corsair Power Supply 750W", 11),
                new(4, "Samsung SSD 512GB", 20)
            ]),

            new(1018, "Şanlıurfa Eyyübiye", "Şanlıurfa",
            [
                new(1, "32GB RAM DDR5", 8),
                new(2, "ElCi UHD Monitor 27\"", 15),
                new(3, "Intel Core i9 12900K", 4),
                new(4, "Envidya Cortex 1000", 12),
                new(5, "NZXT H510 Case", 10)
            ]),

            new(1019, "Balıkesir Altıeylül", "Balıkesir",
            [
                new(1, "ElCi UHD Monitor 24\"", 14),
                new(2, "ElCi UHD Monitor 32\"", 6),
                new(3, "AMDe Rayzen 98 5950X", 7),
                new(4, "Kingston SSD 2TB", 9)
            ]),

            new(1020, "Manisa Yunusemre", "Manisa",
            [
                new(1, "32GB RAM DDR5", 10),
                new(2, "16GB RAM DDR4", 21),
                new(3, "Envidya GTX 2000", 11),
                new(4, "ElCi UHD Monitor 27\"", 13),
                new(5, "Logitech MX Keys", 18)
            ])
        ];
    }
}
