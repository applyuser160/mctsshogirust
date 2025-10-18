use nalgebra::DMatrix;
use ndarray::{Array1, Array2, Axis};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// PCA変換行列を保存するための構造体（ndarray使用）
#[derive(Clone, Debug, PartialEq)]
pub struct PCATransform {
    pub components: Array2<f32>, // 主成分行列
    pub mean: Array1<f32>,       // 平均ベクトル
    pub n_components: usize,     // 主成分数
}

impl PCATransform {
    /// 新しいPCA変換を作成
    pub fn new(components: Array2<f32>, mean: Array1<f32>, n_components: usize) -> Self {
        Self {
            components,
            mean,
            n_components,
        }
    }

    /// 特徴量を変換
    pub fn transform(&self, features: &[f32]) -> Vec<f32> {
        if features.len() != self.mean.len() {
            panic!(
                "Feature dimension mismatch: expected {}, got {}",
                self.mean.len(),
                features.len()
            );
        }

        // 特徴量をndarrayに変換
        let data = Array1::from_vec(features.to_vec());

        // 平均を引く
        let centered = &data - &self.mean;

        // 主成分を適用
        let transformed = self.components.dot(&centered);

        transformed.to_vec()
    }
}

// グローバルなPCA変換を保存
static PCA_TRANSFORM: Lazy<Mutex<Option<PCATransform>>> = Lazy::new(|| Mutex::new(None));

/// グローバルなPCA変換を設定
pub fn set_global_pca_transform(pca_transform: PCATransform) {
    if let Ok(mut transform) = PCA_TRANSFORM.lock() {
        *transform = Some(pca_transform);
    }
}

/// グローバルなPCA変換を取得
pub fn get_global_pca_transform() -> Option<PCATransform> {
    if let Ok(transform) = PCA_TRANSFORM.lock() {
        transform.clone()
    } else {
        None
    }
}

/// 本格的なPCA学習（nalgebra使用）
pub fn learn_pca(samples: &[Vec<f32>], n_components: usize) -> Result<PCATransform, String> {
    if samples.is_empty() {
        return Err("Cannot learn PCA from empty samples".to_string());
    }

    let n_samples = samples.len();
    let n_features = samples[0].len();

    if n_components > n_features {
        return Err(format!(
            "Number of components ({}) cannot be greater than number of features ({})",
            n_components, n_features
        ));
    }

    // データをndarrayに変換
    let mut data = Array2::zeros((n_samples, n_features));
    for (i, sample) in samples.iter().enumerate() {
        for (j, &value) in sample.iter().enumerate() {
            data[[i, j]] = value;
        }
    }

    // 平均を計算
    let mean = data.mean_axis(Axis(0)).unwrap();

    // データを中心化（平均を引く）
    let mut centered = data.clone();
    for mut row in centered.rows_mut() {
        row -= &mean;
    }

    // nalgeb raのDMatrixに変換
    let mut nalgebra_data = DMatrix::zeros(n_samples, n_features);
    for i in 0..n_samples {
        for j in 0..n_features {
            nalgebra_data[(i, j)] = centered[[i, j]];
        }
    }

    // 共分散行列を計算: C = (X^T * X) / (n-1)
    let covariance = nalgebra_data.transpose() * &nalgebra_data / (n_samples - 1) as f32;

    // 固有値分解を実行（nalgeb raの対称固有値分解を使用）
    let eigen = match covariance.symmetric_eigen() {
        eigen => eigen,
    };

    // 固有値と固有ベクトルを取得
    let eigenvalues = eigen.eigenvalues;
    let eigenvectors = eigen.eigenvectors;

    // 固有値の大きい順にソート
    let mut eigenval_vec: Vec<(usize, f32)> = Vec::new();
    for i in 0..eigenvalues.len() {
        eigenval_vec.push((i, eigenvalues[i]));
    }

    eigenval_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // 上位n_components個の主成分を選択
    let mut components = Array2::zeros((n_components, n_features));
    for i in 0..n_components {
        if i < eigenval_vec.len() {
            let idx = eigenval_vec[i].0;
            for j in 0..n_features {
                components[[i, j]] = eigenvectors[(j, idx)];
            }
        }
    }

    Ok(PCATransform::new(components, mean, n_components))
}

/// 簡易的なPCA学習（分散ベースの選択）
pub fn learn_simple_pca(samples: &[Vec<f32>], n_components: usize) -> PCATransform {
    if samples.is_empty() {
        panic!("Cannot learn PCA from empty samples");
    }

    let n_features = samples[0].len();

    // データをndarrayに変換
    let mut data = Array2::zeros((samples.len(), n_features));
    for (i, sample) in samples.iter().enumerate() {
        for (j, &value) in sample.iter().enumerate() {
            data[[i, j]] = value;
        }
    }

    // 平均を計算
    let mean = data.mean_axis(Axis(0)).unwrap();

    // 分散を計算
    let mut variances = Array1::zeros(n_features);
    for row in data.rows() {
        let centered_row = &row - &mean;
        for (i, &value) in centered_row.iter().enumerate() {
            variances[i] += value * value;
        }
    }
    variances /= samples.len() as f32;

    // 分散の大きい順にインデックスをソート
    let mut indices: Vec<usize> = (0..n_features).collect();
    indices.sort_by(|&a, &b| variances[b].partial_cmp(&variances[a]).unwrap());

    // 主成分を作成（簡易実装：単位ベクトル）
    let mut components = Array2::zeros((n_components, n_features));
    for i in 0..n_components {
        if i < indices.len() {
            components[[i, indices[i]]] = 1.0;
        }
    }

    PCATransform::new(components, mean, n_components)
}

/// PCAによる次元圧縮を適用
pub fn apply_pca_compression(features: &[f32], target_dims: usize) -> Vec<f32> {
    if target_dims >= features.len() {
        return features.to_vec();
    }

    // グローバルなPCA変換がある場合はそれを使用
    if let Some(ref transform) = get_global_pca_transform() {
        if transform.n_components == target_dims {
            return transform.transform(features);
        }
    }

    // PCA変換がない場合は簡易的なサンプリングを使用
    let mut compressed = Vec::with_capacity(target_dims);
    let step = features.len() as f32 / target_dims as f32;
    for i in 0..target_dims {
        let index = (i as f32 * step) as usize;
        compressed.push(features[index]);
    }

    compressed
}
