use super::board::Board;
use super::mctsresult::MctsResult;
use super::nn_model::{ModelSaveData, NnModel, NnModelConfig, TrainingConfig, TrainingData};
use burn::backend::ndarray::{NdArray, NdArrayDevice};
use burn::backend::Autodiff;
use serde_json;

/// ニューラルネットワークモデルの使用例
pub fn example_usage() {
    println!("=== ニューラルネットワークモデル使用例 ===");

    // 1. モデル設定を作成
    let model_config = NnModelConfig::default();
    println!(
        "モデル設定: 入力={}, 隠れ層={}, 出力={}",
        model_config.input_dim, model_config.hidden_dim, model_config.output_dim
    );

    // 2. 学習データを作成
    let mut training_data = TrainingData::new();

    // サンプルデータを追加
    for i in 0..10 {
        let mut board = Board::new();
        board.startpos();
        let board_vector = board.to_vector(None);

        // ダミーのターゲットデータ（実際のMCTS結果）
        let target = vec![i as f32, (10 - i) as f32, 10.0];
        training_data.add_sample(board_vector, target);
    }

    println!("学習データサイズ: {}", training_data.len());

    // 3. 学習設定を作成
    let training_config = TrainingConfig {
        learning_rate: 0.001,
        batch_size: 4,
        num_epochs: 5,
        model_save_path: "example_model.bin".to_string(),
    };

    println!(
        "学習設定: 学習率={}, バッチサイズ={}, エポック数={}",
        training_config.learning_rate, training_config.batch_size, training_config.num_epochs
    );

    // 4. モデルを作成（実際のデバイス指定は必要）
    let device = NdArrayDevice::Cpu;
    let model = NnModel::<Autodiff<NdArray>>::new(&model_config, &device);

    // 5. 完全な学習実行（AutodiffBackend使用）
    match model.train(&training_data, &training_config, &device) {
        Ok(trained_model) => {
            println!("完全な学習が完了しました！");

            // 6. モデル保存
            match trained_model.save(&training_config.model_save_path) {
                Ok(_) => println!("モデルを保存しました"),
                Err(e) => println!("保存エラー: {}", e),
            }

            // 7. モデル読み込み
            match NnModel::<Autodiff<NdArray>>::load(&training_config.model_save_path, &device) {
                Ok(mut loaded_model) => {
                    println!("モデルを読み込みました");

                    // 8. 重み設定の例
                    let weights = loaded_model.get_weights();
                    loaded_model.set_weights(weights, &device);
                    println!("重み設定が完了しました");
                }
                Err(e) => println!("読み込みエラー: {}", e),
            }
        }
        Err(e) => println!("学習エラー: {}", e),
    }

    println!("=== 使用例完了 ===");

    // 追加の使用例を実行
    example_weight_setting();
    example_full_training_pipeline();
}

/// MctsResultから学習データを作成する例
pub fn create_training_data_from_mcts_results(mcts_results: Vec<MctsResult>) -> TrainingData {
    let mut training_data = TrainingData::new();

    for result in mcts_results {
        // 盤面をベクターに変換
        let board_vector = result.board.to_vector(None);

        // MCTS結果をターゲットに変換
        let target = vec![
            result.white_wins as f32,
            result.black_wins as f32,
            result.total_games as f32,
        ];

        training_data.add_sample(board_vector, target);
    }

    training_data
}

/// 学習データの統計情報を表示
pub fn print_training_data_stats(training_data: &TrainingData) {
    if training_data.is_empty() {
        println!("学習データが空です");
        return;
    }

    println!("=== 学習データ統計 ===");
    println!("データサイズ: {}", training_data.len());

    // 入力データの統計
    let first_input = &training_data.inputs[0];
    println!("入力次元: {}", first_input.len());

    // ターゲットデータの統計
    let mut total_white_wins = 0.0;
    let mut total_black_wins = 0.0;
    let mut total_games = 0.0;

    for target in &training_data.targets {
        total_white_wins += target[0];
        total_black_wins += target[1];
        total_games += target[2];
    }

    println!(
        "平均白勝利数: {:.2}",
        total_white_wins / training_data.len() as f32
    );
    println!(
        "平均黒勝利数: {:.2}",
        total_black_wins / training_data.len() as f32
    );
    println!(
        "平均総ゲーム数: {:.2}",
        total_games / training_data.len() as f32
    );
    println!("==================");
}

/// モデル保存・読み込みの使用例
pub fn example_save_load() {
    println!("=== モデル保存・読み込み使用例 ===");

    // 1. モデル保存データを作成
    let save_data = ModelSaveData {
        config: NnModelConfig::default(),
        input_layer_weights: vec![vec![1.0; 2320]; 512],
        input_layer_bias: vec![0.0; 512],
        output_layer_weights: vec![vec![1.0; 512]; 3],
        output_layer_bias: vec![0.0; 3],
    };

    // 2. JSON形式で保存
    let json_data = serde_json::to_string_pretty(&save_data).unwrap();
    std::fs::write("example_model.json", json_data).unwrap();
    println!("モデルをJSON形式で保存しました");

    // 3. JSON形式で読み込み
    let loaded_json = std::fs::read_to_string("example_model.json").unwrap();
    let loaded_data: ModelSaveData = serde_json::from_str(&loaded_json).unwrap();
    println!("モデルをJSON形式で読み込みました");

    // 4. 読み込んだデータの確認
    println!("読み込んだ重みのサイズ:");
    println!(
        "  入力層重み: {} x {}",
        loaded_data.input_layer_weights.len(),
        loaded_data.input_layer_weights[0].len()
    );
    println!(
        "  出力層重み: {} x {}",
        loaded_data.output_layer_weights.len(),
        loaded_data.output_layer_weights[0].len()
    );

    // 5. テストファイルを削除
    std::fs::remove_file("example_model.json").unwrap();
    println!("テストファイルを削除しました");

    println!("=== 保存・読み込み例完了 ===");
}

/// 重み設定機能の使用例
pub fn example_weight_setting() {
    println!("=== 重み設定機能使用例 ===");

    // 1. モデル設定を作成
    let model_config = NnModelConfig::default();

    // 2. カスタム重みデータを作成
    let custom_weights = ModelSaveData {
        config: model_config.clone(),
        input_layer_weights: vec![vec![0.5; 2320]; 512], // カスタム重み
        input_layer_bias: vec![0.1; 512],                // カスタムバイアス
        output_layer_weights: vec![vec![0.3; 512]; 3],
        output_layer_bias: vec![0.2; 3],
    };

    println!("カスタム重みデータを作成しました");
    println!(
        "入力層重み: {} x {}",
        custom_weights.input_layer_weights.len(),
        custom_weights.input_layer_weights[0].len()
    );
    println!(
        "出力層重み: {} x {}",
        custom_weights.output_layer_weights.len(),
        custom_weights.output_layer_weights[0].len()
    );

    // 3. モデル作成と重み設定
    let device = NdArrayDevice::Cpu;
    let mut model = NnModel::<Autodiff<NdArray>>::new(&model_config, &device);
    model.set_weights(custom_weights, &device);
    println!("重み設定が完了しました");

    // 4. 重みの確認
    let current_weights = model.get_weights();
    println!("現在の重み:");
    println!(
        "  入力層重みの最初の値: {}",
        current_weights.input_layer_weights[0][0]
    );
    println!(
        "  出力層重みの最初の値: {}",
        current_weights.output_layer_weights[0][0]
    );

    println!("=== 重み設定例完了 ===");
}

/// 完全な学習パイプラインの例
pub fn example_full_training_pipeline() {
    println!("=== 完全な学習パイプライン例 ===");

    // 1. 学習データの準備
    let mut training_data = TrainingData::new();

    // サンプルデータを追加
    for i in 0..20 {
        let mut board = Board::new();
        board.startpos();
        let board_vector = board.to_vector(None);

        // より現実的なターゲットデータ
        let white_wins = (i % 3) as f32;
        let black_wins = ((i + 1) % 3) as f32;
        let total_games = 3.0;
        let target = vec![white_wins, black_wins, total_games];

        training_data.add_sample(board_vector, target);
    }

    println!("学習データを準備しました: {} サンプル", training_data.len());

    // 2. 学習設定
    let training_config = TrainingConfig {
        learning_rate: 0.001,
        batch_size: 8,
        num_epochs: 10,
        model_save_path: "trained_model.json".to_string(),
    };

    println!(
        "学習設定: 学習率={}, バッチサイズ={}, エポック数={}",
        training_config.learning_rate, training_config.batch_size, training_config.num_epochs
    );

    // 3. 完全な学習パイプライン
    let device = NdArrayDevice::Cpu;
    let model_config = NnModelConfig::default();
    let model = NnModel::<Autodiff<NdArray>>::new(&model_config, &device);

    // 学習実行
    match model.train(&training_data, &training_config, &device) {
        Ok(trained_model) => {
            println!("学習が完了しました！");

            // モデル保存
            match trained_model.save(&training_config.model_save_path) {
                Ok(_) => println!("学習済みモデルを保存しました"),
                Err(e) => println!("保存エラー: {}", e),
            }

            // モデル読み込みとテスト
            match NnModel::<Autodiff<NdArray>>::load(&training_config.model_save_path, &device) {
                Ok(loaded_model) => {
                    println!("学習済みモデルを読み込みました");

                    // テストデータでの予測
                    let mut test_board = Board::new();
                    test_board.startpos();
                    let test_vector = test_board.to_vector(None);

                    // 予測実行
                    let prediction = loaded_model.predict_single(test_vector);
                    println!("予測結果: {:?}", prediction);
                }
                Err(e) => println!("読み込みエラー: {}", e),
            }
        }
        Err(e) => println!("学習エラー: {}", e),
    }

    println!("=== 完全な学習パイプライン例完了 ===");
}
