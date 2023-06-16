use crate::renderable::HitRecord;
use crate::ray::Ray;
use crate::util::{Color, Vec3, Point, random_between_0_1};
use std::fmt;
pub trait Material: fmt::Display {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (bool, Color, Ray);
}

#[derive(Debug, Copy, Clone)]
pub struct LambertianMaterial { 
    albedo: Color
}

// default material 
impl LambertianMaterial {
    pub fn new(albedo: Color) -> Self {
        Self {albedo}
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (bool, Color, Ray) {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.point, scatter_direction);
        (true, self.albedo, scattered)
    }
}

impl fmt::Display for LambertianMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\t\t\"material_type\": \"lambertian\",\n\t\t\"albedo\": {}", self.albedo)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzziness: f32
}

impl Metal {
    pub fn new(color: Color, fuzziness: Option<f32>) -> Self {
        let f = fuzziness.unwrap_or(0.0);
        Self { albedo: color, fuzziness: if f < 1.0 { f } else { 0.0 } } 
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (bool, Color, Ray) {
        let reflected = Vec3::reflect(r_in.direction.unit_vector(), hit_record.normal);
        /*
        * if there is any fuzziness ( > 0.0) then it will add some offset in a unit sphere 
        * around where the actual reflection would be
        */
        let scattered = Ray::new(
            hit_record.point, 
            reflected + (self.fuzziness * Point::random_in_unit_sphere())
        );
        return (scattered.direction.dot(hit_record.normal) > 0.0, self.albedo, scattered);
    }
}

impl fmt::Display for Metal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\t\t\"material_type\": \"metal\",\n\t\t\"albedo\": {},\n\t\t\"fuzziness\": {}", self.albedo, self.fuzziness)
    }
}

pub struct Dielectric {
    // index of refraction (𝜂')
    ir: f32
}

impl Dielectric {
    pub fn new(index_of_refraction: Option<f32>) -> Self {
        Self { ir: index_of_refraction.unwrap_or(1.0) }
    }

    /**
     * Real glass reflects differently depending on what angle you're looking at
     * This is a polynomial approximation of it done by Christopher Schlick
     */
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        return r0 * (1.0 - r0) * (f32::powi(1.0 - cosine, 5));
    }
}

impl Material for Dielectric {
    /**
     * As the equation for refraction given Snell's Law is:
     * 𝜂 * sin(theta) = 𝜂' * sin(theta')
     * if our incoming ray is within a material with a higher refraction index (𝜂 > 𝜂'), then refraction is impossible
     * Why?
     * If we solve for sin(theta'), we rearrange Snell's Law to:
     * 𝜂 / 𝜂' * sin(theta) = sin(theta')
     * in the cases where our current material is higher than our outside material (𝜂 > 𝜂'), we get 𝜂/𝜂' to be > 1.0
     * As sin(theta') can at most be = 1.0, that means if the lhs of the equation is > 1.0, then refraction cannot occur
     * Therefore, we have to reflect instead of refract
     * 
     * We can solve for sin(theta) using trig identities (remember sin(theta) is the sin of the angle of incidence), to yield
     * sin(theta) = sqrt(1 - cos(theta)^2)
     * And we know that cos(theta) is equal to R * n, given R and n are both unit vectors
     * so our final solution for sin(theta) = sqrt(1 - |R * n|^2) 
     */
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (bool, Color, Ray) {
        // doesn't dim the reflection at all, full brightness and full color
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face { 1.0 / self.ir } else { self.ir };
        let unit_direction = Vec3::unit_vector(&r_in.direction);
        
        let cos_theta = -r_in.direction.dot(hit_record.normal);
        let sin_theta = f32::sqrt(1.0 - cos_theta*cos_theta);
        let cannot_refract = refraction_ratio * sin_theta <= 1.0;
        // assume by default that it won't refract, so reflect it instead
        let mut direction = Vec3::reflect(unit_direction, hit_record.normal);
        // some angles of viewing produce higher reflection than others
        if cannot_refract || Self::reflectance(cos_theta, self.ir) > random_between_0_1(){
            let refracted = Vec3::refract(unit_direction, hit_record.normal, refraction_ratio);
            direction = refracted;
        }
        // refraction can, and (and so it does) occur
            
        let scattered = Ray::new(hit_record.point, direction);
        (true, attenuation, scattered)
    }
}

impl fmt::Display for Dielectric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\t\t\"material_type\": \"dielectric\",\n\t\t\"ir\": {}", self.ir)
    }
}
